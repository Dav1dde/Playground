#!rdmd

private {
    import std.algorithm : max, map, filter, endsWith, canFind;
    import std.path : buildPath, extension, setExtension, dirName, baseName, getcwd, chdir, buildNormalizedPath;
    import std.array : join, split, appender, array;
    import std.process : shell, system, environment;
    import std.file : dirEntries, SpanMode, mkdirRecurse, rmdirRecurse, FileException, exists, copy, remove, readText, write;
    import std.stdio : writeln, writefln, File;
    import std.string : format, stripRight;
    import std.parallelism : TaskPool;
    import std.exception : enforce, collectException;
    import std.typetuple : TypeTuple;
    import std.getopt;
    import std.digest.md;
    import std.digest.digest;
    import core.time : dur;
    import std.json;

    version(linux) {
        import std.path : symlink;
    }
}

version(Windows) {
    enum OBJ = ".obj";
    enum SO = ".dll";
    enum SO_LINK = ".lib";
} else version(linux) {
    enum OBJ = ".o";
    enum SO = ".so";
    enum SO_LINK = "";
} else version(OSX) {
    enum OBJ = ".o";
    enum SO = ".dylib";
    enum SO_LINK = "";
}


alias filter!("a.length > 0") filter0;

bool is32bit() {
    return is(uint == size_t);
}
unittest {
    if(is32bit()) {
        assert(!is(ulong == size_t));
    } else {
        assert(is(ulong == size_t));
    }
}

abstract class Compiler {
    string build_prefix;
    string[] import_paths;
    string[] additional_flags;

    @property string import_flags() {
        return filter0(import_paths).map!(x => "-I" ~ x).join(" ");
    }

    @property string compiling_ext() { throw new Exception("Not Implemented"); }
    void compile(string prefix, string file) { throw new Exception("Not Implemented"); }

    string version_(string ver) { throw new Exception("Not Implemented"); }
    @property string debug_() { throw new Exception("Not Implemented"); }
    @property string debug_info() { throw new Exception("Not Implemented"); }

    @property string compiler() { throw new Exception("Not Implemented"); }

    bool is_available() {
        return system("%s -v".format(compiler)) == 0;
    }
}

class DCompiler : Compiler {
    override @property string compiling_ext() {
        return ".d";
    }
}

class CCompiler : Compiler {
    override @property string compiling_ext() {
        return ".c";
    }
}


interface Linker {
    void link(string, string[], string[], string[]);
}

class DMD : DCompiler, Linker {
    override string version_(string ver) {
        return "-version=" ~ ver;
    }
    override @property string debug_() { return "-debug"; }
    override @property string debug_info() { return "-g -gc"; }
    override @property string compiler() { return "dmd"; }
    
    override void compile(string src, string dest) {
        string cmd = "dmd %s %s -c %s -of%s".format(import_flags, filter0(additional_flags).join(" "), src, dest);
        writeln(cmd);
        shell(cmd);
    }

    void link(string out_path, string[] object_files, string[] libraries, string[] options) {
        enum LF = "-L";
        version(Windows) {
            enum LLF = "";
        } else {
            enum LLF = "-L-l";
        }
        
        string cmd = "dmd %s %s %s -of%s".format(
                        object_files.join(" "),
                        filter0(libraries).map!(x => LLF ~ x).join(" "),
                        filter0(options).map!(x => LF ~ x).join(" "),
                        out_path);

        writeln(cmd);
        shell(cmd);
    }
}

// GDC: "-fdebug -g";
// LDC: "-debug -g -gc";

class GDC : DCompiler {
}

class LDC : DCompiler {
}

class DMC : CCompiler {
    override @property string compiler() { return "dmc"; }
    override void compile(string src, string dest) {
        string cmd = "dmc %s %s -c %s -o%s".format(import_flags, filter0(additional_flags).join(" "), src, dest);
        writeln(cmd);
        shell(cmd);
    }
}

class GCC : CCompiler {
    override @property string compiler() { return "gcc"; }
    override void compile(string src, string dest) {
        string cmd = "gcc %s %s -c %s -o %s".format(import_flags, filter0(additional_flags).join(" "), src, dest);
        writeln(cmd);
        shell(cmd);
    }
}

Compiler get_compiler(string compiling_ext, string name) {
    if(compiling_ext == ".d" || compiling_ext == "d") {
        if(name.length) {
            switch(name) {
                case "dmd": return new DMD();
                case "gdc": return new GDC();
                case "ldc": return new LDC();
                default: throw new Exception("No D compiler with name %s".format(name));
            }
        } else {
            version(DigitalMars) {
                return new DMD();
            } else version(GNU) {
                return new GDC();
            } else version(LDC) {
                return new LDC();
            } else {
                throw new Exception("Unsupported D compiler used");
            }
        }
    } else if(compiling_ext == ".c" || compiling_ext == "c") {
        if(name.length) {
            switch(name) {
                case "dmc": return new DMC();
                case "gcc": return new GCC();
                default: throw new Exception("No C compiler with name %s".format(name));
            }
        } else {
            version(Windows) {
                return new DMC();
            } else {
                return new GCC();
            }
        }
    } else {
        throw new Exception("No compiler found for extension %s and name %s.".format(compiling_ext, name));
    }
}

interface Cache {
    bool is_cached(string src, string dest);
    void add_file_to_cache(string file);
}

class NoCache : Cache {
    bool is_cached(string src, string dest) { return false; }
    void add_file_to_cache(string file) {}
}

class MD5Cache : Cache {
    string[string] cache;

    this()() {}

    this(T)(T c) if(is(T : string) || is(T : File) || is(T : string[])) {
        load(c);
    }

    void load(string cache_file) {
        if(cache_file.exists()) {
            File f = File(cache_file, "r");
            scope(exit) f.close();

            load(f);
        }
    }

    void load(File cache_file) {
        _add_to_cache(cache_file.byLine());
    }

    void load(string[] cache_file) {
        _add_to_cache(cache_file);
    }

    private void _add_to_cache(T)(T iter) {
        foreach(raw_line; iter) {
            if(!raw_line.length) {
                continue;
            }

            static if(!is(typeof(line) : string)) {
                string line = raw_line.idup;
            } else {
                alias raw_line line;
            }
            
            string hash = line.split()[$-1];
            string file = line[0..$-hash.length].stripRight();

            cache[file] = hash;
        }
    }

    void add_file_to_cache(string file) {
        cache[file] = MD5Cache.hexdigest_from_file(file);
    }

    bool is_cached(string src, string dest) {
        if(src in cache && dest.exists()) {
            string hexdigest = MD5Cache.hexdigest_from_file(src);

            return cache[src] == hexdigest;
        }

        return false;
    }

    void save(string file) {
        File f = File(file, "w");
        scope(exit) f.close();

        save(f);
    }

    void save(File file) {
        foreach(k, v; cache) {
            file.writefln("%s %s", k, v);
        }
    }

    static string hexdigest_from_file(Hash = MD5)(string file) {
        File f = File(file, "r");
        scope(exit) f.close();

        return hexdigest_from_file!(Hash)(f);
    }

    static string hexdigest_from_file(Hash = MD5)(File file) {
        auto b = file.byChunk(4096 * 1024);
        return hexDigest!Hash(b).idup;
    }
}


struct ScanPath {
    string path;
    alias path this;
    SpanMode mode;
}

class Builder {
    ScanPath[] scan_paths;
    protected string[] _object_files;
    @property string[] object_files() { return _object_files; }

    protected Compiler[string] compiler;
    Linker linker;
    Cache cache;
    
    string out_path;
    string out_file;

    @property string[] library_paths() {
        version(Windows) {
            return [buildPath("lib", "win"), buildPath("lib", "win%s".format(is32bit() ? "32" : "64"))];
        } else version(linux) {
            return [buildPath("lib", "linux"), buildPath("lib", "linux%s".format(is32bit() ? "32" : "64"))];
        } else version(OSX) {
            return [buildPath("lib", "osx"), buildPath("lib", "osx%s".format(is32bit() ? "32" : "64"))];
        }
    }

    string[] libraries_windows;
    string[] libraries_windows32;
    string[] libraries_windows64;
    string[] libraries_linux;
    string[] libraries_linux32;
    string[] libraries_linux64;
    string[] libraries_osx;
    string[] libraries_osx32;
    string[] libraries_osx64;

    @property string[] libraries() {
        version(Windows) {
            static if(is32bit()) {
                return libraries_windows ~ libraries_windows32;
            } else {
                return libraries_windows ~ libraries_windows64;
            }
        } else version(linux) {
            static if(is32bit()) {
                return libraries_linux ~ libraries_linux32;
            } else {
                return libraries_linux ~ libraries_linux64;
            }
        } else version(OSX) {
            static if(is32bit()) {
                return libraries_osx ~ libraries_osx32;
            } else {
                return libraries_osx ~ libraries_osx64;
            }
        }
    }

    string[] linker_options_windows;
    string[] linker_options_windows32;
    string[] linker_options_windows64;
    string[] linker_options_linux;
    string[] linker_options_linux32;
    string[] linker_options_linux64;
    string[] linker_options_osx;
    string[] linker_options_osx32;
    string[] linker_options_osx64;

    @property string[] linker_options() {
        version(Windows) {
            static if(is32bit()) {
                return linker_options_windows ~ linker_options_windows32;
            } else {
                return linker_options_windows ~ linker_options_windows64;
            }
        } else version(linux) {
            static if(is32bit()) {
                return linker_options_linux ~ linker_options_linux32;
            } else {
                return linker_options_linux ~ linker_options_linux64;
            }
        } else version(OSX) {
            static if(is32bit()) {
                return linker_options_osx ~ linker_options_osx32;
            } else {
                return linker_options_osx ~ linker_options_osx64;
            }
        }
    }

    string build_prefix;

    this() {
        // TODO: implement find_compiler
        version(Windows) {
            auto dc = new DMD();
            auto cc = new DMC();
            this(new NoCache(), dc, dc, cc);
        } else {
            auto dc = new DMD();
            auto cc = new GCC();
            this(new NoCache(), dc, dc, cc);
        }
    }

    this(Cache cache, Linker linker, Compiler[] compiler...) {
        this.cache = cache;
        this.linker = linker;

        foreach(c; compiler) {
            this.compiler[c.compiling_ext] = c;
        }
    }

    void add_scan_path(string path, SpanMode mode = SpanMode.breadth) {
        scan_paths ~= ScanPath(path, mode);
    }
    void add_scan_path(ScanPath scan_path) {
        scan_paths ~= scan_path;
    }

    void compile(TaskPool task_pool=null) {
        if(scan_paths.length == 0) {
            throw new Exception("No files to compile");
        }

        foreach(path; scan_paths) {
            auto files = map!(x => x.name)(filter!(e => compiler.keys.canFind(e.name.extension))(dirEntries(path, path.mode))).array();

            enum foreach_body = "collectException(mkdirRecurse(buildPath(build_prefix, file.dirName())));
                                 Compiler compiler = this.compiler[file.extension];
                                 string dest = buildPath(build_prefix, file).setExtension(OBJ);
                                 if(!cache.is_cached(file, dest)) {
                                     compiler.compile(file, dest);
                                     cache.add_file_to_cache(file);
                                 }
                                 _object_files ~= dest;";

            if(task_pool is null) {
                foreach(file; files) {
                    mixin(foreach_body);
                }
            } else {
                size_t work_units = max(files.length / task_pool.size, 1);
                
                foreach(file; task_pool.parallel(files, work_units)) {
                    mixin(foreach_body);
                }
            }
        }
    }

    void link() {
        linker.link(buildPath(out_path, out_file), _object_files, libraries, linker_options);
    }
    
    static Builder from_json(string file, Cache cache = null) {
        auto json = parseJSON(file.readText());
        
        auto compiler = json["compiler"].object;
        
        if("d" !in compiler) {
            throw new Exception("Need at least a D compiler in build.json");
        }
        
        Compiler[] compiler_objects;
        Linker linker = new DMD(); // TODO: support other linkers
        
        foreach(name; compiler.byKey()) {
            auto compiler_data = compiler[name].object;
            
            string cname = "name" in compiler_data ? compiler_data["name"].str : "";
            auto compiler_object = get_compiler(name, cname);
            
            auto additional_flags = compiler_data["additional_flags"].array.map!(x => x.str).array;
            auto import_paths = compiler_data["import_paths"].array.map!(x => x.str)().array;
            
            compiler_object.additional_flags = additional_flags;
            compiler_object.import_paths = import_paths;
            
            if("version_flags" in compiler_data) {
                foreach(jversion_flag; compiler_data["version_flags"].array) {
                    compiler_object.additional_flags ~= compiler_object.version_(jversion_flag.str);
                }
            }
            
            compiler_objects ~= compiler_object;
        }
        
        if(cache is null) {
            cache = new NoCache();
        }
        
        auto jbuilder = json["builder"].object;
        
        auto builder = new Builder(cache, linker, compiler_objects);
        
        builder.out_path = jbuilder["out_path"].str;
        builder.out_file = jbuilder["out_file"].str;
        builder.build_prefix = jbuilder["build_prefix"].str;
        
        foreach(jscan_path; jbuilder["scan_paths"].array) {
            builder.add_scan_path(jscan_path.str.buildNormalizedPath());            
        }
        
        auto jlibraries = jbuilder["libraries"].object;
        foreach(name; TypeTuple!("windows", "windows32", "windows64",
                                 "linux", "linux32", "linux64",
                                 "osx", "osx32", "osx64")) {
            mixin(`builder.libraries_` ~ name ~ ` = jlibraries["` ~ name ~ `"].array.map!(x => x.str)().array;`);
        }
        
        auto jlinker = jbuilder["linker"].object;
        foreach(name; TypeTuple!("windows", "windows32", "windows64",
                                 "linux", "linux32", "linux64",
                                 "osx", "osx32", "osx64")) {
            mixin(`builder.linker_options_` ~ name ~ ` = jlinker["` ~ name ~ `"].array.map!(x => x.str)().array;`);
        }

        return builder;
    }
}


int main(string[] args) {
    size_t jobs = 1;
    string cache_file = ".build_cache";
    bool no_cache = false;
    bool override_cache = false;
    string json_file = "build.json";
    getopt(args, "jobs|j", &jobs,
                 "cache|c", &cache_file,
                 "no-cache", &no_cache,
                 "override-cache|o", &override_cache,
                 "json", &json_file);
    enforce(jobs >= 1, "Jobs can't be 0 or negative");

    string project;
    if(args.length > 1) {
        if(args.length != 2) {
            writefln("Can only process one project at a time.");
            return 1;
        }
                
        project = args[1];
        json_file = buildPath(project, json_file);
    }
    
    if(!json_file.exists()) {
        writefln(`No Buildfile found at: "%s"`, json_file);
        return 1;
    }   

    TaskPool task_pool;
    if(jobs > 1) {
        task_pool = new TaskPool(jobs);
    }
    scope(exit) { if(task_pool !is null) task_pool.finish(); }


    MD5Cache md5_cache = new MD5Cache();
    if(!override_cache) {
        md5_cache.load(cache_file);
    }
    
    Cache cache;
    if(no_cache) {
        cache = new NoCache();
    } else {
        cache = md5_cache;
    }

    auto builder = Builder.from_json(json_file, cache);

    collectException(rmdirRecurse(builder.out_file));
    
    builder.compile(task_pool);
    builder.link();
    
    if(!no_cache) {
        md5_cache.save(cache_file);
    }

    return 0;
}