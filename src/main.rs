pub mod ui;

fn main() {
    let a = '\t';
    //ui::meta::generate_meta();
    //ui::compile();
    let xml_input = r#"
        <label>Hello</label>
        <button type="button" onclick="myfunction">Click me</button>
        <label myAttrib=10 />
        <element>
            <label>Hi</label>
        </element>
    "#;
    let tokens = ui::xml::lexer::tokenize(xml_input.to_string());
    let tree = ui::xml::parser::parse(tokens);
    let generated = ui::xml::codegen::generate(tree);
    println!("{}", generated);
}
