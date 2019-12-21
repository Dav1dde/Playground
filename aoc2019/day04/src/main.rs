
type Password = i64;

trait Rule {
    fn matches(&self, password: Password) -> bool;
}


struct AdjacentDigitsRule {
}

impl Rule for AdjacentDigitsRule {
    fn matches(&self, password: Password) -> bool {
        let mut last = None;
        for c in password.to_string().chars() {
            if Some(c) == last {
                return true;
            }
            last = Some(c);
        }

        false
    }
}

impl AdjacentDigitsRule {
    pub fn new() -> Self {
        AdjacentDigitsRule {}
    }
}

struct DoubleAdjacentDigitsRule {
}

impl Rule for DoubleAdjacentDigitsRule {
    fn matches(&self, password: Password) -> bool {
        let mut last = None;
        let mut repetition = 0;
        for c in password.to_string().chars() {
            if Some(c) == last {
                repetition = repetition + 1;
            } else {
                if repetition == 1 {
                    return true;
                }
                repetition = 0;
            }
            last = Some(c);
        }

        repetition == 1
    }
}

impl DoubleAdjacentDigitsRule {
    fn new() -> Self {
        DoubleAdjacentDigitsRule {}
    }
}


struct GrowingRule {
}

impl Rule for GrowingRule {
    fn matches(&self, password: Password) -> bool {
        let mut last = -1;

        for c in password.to_string().chars() {
            let n = c.to_string().parse().unwrap();
            if n < last {
                return false;
            }
            last = n;
        }

        true
    }
}

impl GrowingRule {
    pub fn new() -> Self {
        GrowingRule {}
    }
}


struct PasswordGen {
    rules: Vec<Box<dyn Rule>>
}

impl PasswordGen {
    pub fn new() -> Self {
        PasswordGen { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }

    pub fn generate(&self, from: Password, to: Password) -> Vec<Password> {
        let mut passwords = Vec::new();

        for password in from..to {
            let matches = self.rules.iter().all(|rule| rule.matches(password));
            if matches {
                passwords.push(password);
            }
        }

        passwords
    }
}


fn main() {
    let mut gen = PasswordGen::new();
    gen.add_rule(Box::new(GrowingRule::new()));
    gen.add_rule(Box::new(AdjacentDigitsRule::new()));

    let passwords = gen.generate(152085, 670283);
    println!("found {} passwords", passwords.len());

    gen.add_rule(Box::new(DoubleAdjacentDigitsRule::new()));
    let passwords = gen.generate(152085, 670283);
    println!("found {} passwords with double adjacent rule", passwords.len());
}

