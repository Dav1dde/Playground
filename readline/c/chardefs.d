module lib.readline.c.chardefs;


bool whitespace(T)(T c) { return (c == ' ') || (c == '\t'); }


enum control_character_threshold = 0x020; /* Smaller than this is control. */
enum control_character_mask = 0x1f;       /* 0x20 - 1 */
enum meta_character_threshold = 0x07f;    /* Larger than this is Meta. */
enum control_character_bit = 0x40;        /* 0x000000, must be off. */
enum meta_character_bit = 0x080;          /* x0000000, must be on. */
enum largest_char = 255;                  /* Largest character value. */

bool CTRL_CHAR(T)(T c) { return (c < control_character_threshold) && ((c & 0x80) == 0); }
bool META_CHAR(T)(T c) { return (c > meta_character_threshold) && (c <= largest_char); }


T CTRL(T)(T x) { return x & control_character_mask; }
T META(T)(T x) { return x | meta_character_bit; }

T UNMETA(T)(T x) { return x & (~meta_character_bit); }
// doesn't match the C header
T UNCTRL(T)(T x) { return x + 64; }

enum NEWLINE = '\n';
enum RETURN = CTRL('M');
enum RUBOUT = 0x7f;
enum TAB = '\t';
enum ABORT_CHAR = CTRL('G');
enum PAGE = CTRL('L');
enum SPACE = ' '; /* XXX - was 0x20 */
enum ESC = CTRL('[');
