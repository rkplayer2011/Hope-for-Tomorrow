fn main() {
    println!("Hope for tomorrow");

    let mut context = Context::new();
    context.add_rule("will have a bright future", 0);
    context.add_rule("will be better", 0);
    context.add_rule("will be a better tomorrow", 0);
    context.add_rule("will be more peaceful", 0);

    let mut state = State::new("Hope for tomorrow");
    state.add_context(&context);
    state.run_rules();

    let results = state.gen_results();
    println!("{:#?}", results);

    let mut state = State::new("Hope for tomorrow");
    for phrase in &results {
        state.add_sentence(phrase);
    }
    state.run_rules();
    let results = state.gen_results();
    println!("{:#?}", results);

    let mut state = State::new("Hope for tomorrow");
    for phrase in &results {
        state.add_sentence(phrase);
    }
    state.run_rules();
    let results = state.gen_results();
    println!("{:#?}", results);

    let mut context = Context::new();
    context.add_rule("can be achieved through hard work", 0);
    context.add_rule("is within our reach", 0);
    context.add_rule("lies in our collective action", 0);
    context.add_rule("lies in our will to make it happen", 0);

    let mut state = State::new("Hope for tomorrow");
    state.add_context(&context);
    for phrase in &results {
        state.add_sentence(phrase);
    }
    state.run_rules();

    let results = state.gen_results();
    println!("{:#?}", results);

    let mut state = State::new("Hope for tomorrow");
    for phrase in &results {
        state.add_sentence(phrase);
    }
    state.run_rules();
    let results = state.gen_results();
    println!("{:#?}", results);

    let mut context = Context::new();
    context.add_rule("can lift us out of despair", 0);
    context.add_rule("will lead to a more equitable tomorrow", 0);
    context.add_rule("will lead to a more just future", 0);
    context.add_rule("will lead to a more prosperous world", 0);

    let mut state = State::new("Hope for tomorrow");
    state.add_context(&context);
    for phrase in &results {
        state.add_sentence(phrase);
    }
    state.run_rules();

    let results = state.gen_results();
    println!("{:#?}", results);

    let mut state = State::new("Hope for tomorrow");
    for phrase in &results {
        state.add_sentence(phrase);
    }
    state.run_rules();
    let results = state.gen_results();
    println!("{:#?}", results);

    let mut context = Context::new();
    context.add_rule("is what will make us strive forward", 0);
    context.add_rule("will help us keep our heads held high", 0);
    context.add_rule("can help us overcome any challenge", 0);
    context.add_rule("is what will keep us moving forward", 0);

    let mut state = State::new("Hope for tomorrow");
    state.add_context(&context);
    for phrase in &results {
        state.add_sentence(phrase);
    }
    state.run_rules();

    let results = state.gen_results();
    println!("{:#?}", results);

    let mut state = State::new("Hope for tomorrow");
    for phrase in &results {
        state.add_sentence(phrase);
    }
    state.run_rules();
    let results = state.gen_results();
    println!("{:#?}", results);

    let mut context = Context::new();
    context.add_rule("is our only option for a better future", 0);
    context.add_rule("is the light at the end of the tunnel", 0);
    context.add_rule("is what will keep us going in tough times", 0);
    context.add_rule("is our reassurance in the face of adversity", 0);

    let mut state = State::new("Hope for tomorrow");
    state.add_context(&context);
    for phrase in &results {
        state.add_sentence(phrase);
    }
    state.run_rules();

    let results = state.gen_results();
    println!("{:#?}", results);
}

struct Context {
    rules: Vec<(String, i32)>,
}
 
impl Context {
    fn new() -> Self {
        Context { rules: vec![] }
    }
    fn add_rule(&mut self, rule: &str, weight: i32) {
        self.rules.push((rule.to_string(), weight));
    }
}
 
struct State {
    sentence: String,
    context: Context,
    sentences: Vec<String>,
}
 
impl State {
    fn new(sentence: &str) -> Self {
        State {
            sentence: sentence.to_string(),
            context: Context::new(),
            sentences: vec![sentence.to_string()],
        }
    }
    fn add_context(&mut self, context: &Context) {
        self.context = context.clone();
    }
    fn add_sentence(&mut self, sentence: &str) {
        self.sentences.push(sentence.to_string());
    }
    fn run_rules(&mut self) {
        for (rule, weight) in &self.context.rules {
            for sentence in &mut self.sentences {
                if sentence.contains(rule) {
                    *sentence += &format!(" {} ({})", rule, weight);
                }
            }
        }
    }
    fn gen_results(&self) -> Vec<String> {
        self.sentences.clone()
    }
}