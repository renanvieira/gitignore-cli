use include_dir::Dir;

pub fn build_output(selection: Vec<&str>, template_dir: &Dir) -> String {
    let mut output: String = String::new();

    for tech in selection {
        if let Some(ignore_content) = template_dir.get_file(format!("{}.gitignore", tech)) {
            output.push_str(&format!(
            "\n\n######### Start of automatic generated ignores for {} ##########\n\n{}\n######### End of automatic generated ignores for {} ##########\n",
            tech,
            ignore_content.contents_utf8().unwrap(),
            tech
        ));
        }
    }

    output
}
