use pandoc::{ self, OutputKind, PandocOutput };
use pandoc_ast::{ Pandoc, Inline, Block };

pub fn stringify_inline(inline: &Inline) -> String {
    match inline {
        Inline::Str(text) => text.clone(),
        | Inline::Emph(children)
        | Inline::Strong(children)
        | Inline::Strikeout(children)
        | Inline::Superscript(children)
        | Inline::Subscript(children)
        | Inline::SmallCaps(children)
        | Inline::Quoted(_, children)
        | Inline::Cite(_, children)
        | Inline::Link(_, children, _)
        | Inline::Image(_, children, _)
        | Inline::Span(_, children) => {
            children.iter().map(stringify_inline).collect::<Vec<_>>().join("")
        }
        Inline::Space => " ".to_string(),
        Inline::SoftBreak | Inline::LineBreak => "\n".to_string(),
        Inline::Code(_, code) => code.clone(),
        Inline::Math(_, content) => content.clone(),
        Inline::RawInline(_, content) => content.clone(),
        Inline::Note(blocks) => blocks.iter().map(stringify_block).collect::<Vec<_>>().join("\n"),
        _ => String::new(),
    }
}

pub fn stringify_block(block: &pandoc_ast::Block) -> String {
    match block {
        Block::Plain(children) | Block::Para(children) => {
            children.iter().map(stringify_inline).collect::<Vec<_>>().join("")
        }
        Block::LineBlock(lines) => {
            lines
                .iter()
                .map(|line| line.iter().map(stringify_inline).collect::<Vec<_>>().join(""))
                .collect::<Vec<_>>()
                .join("\n")
        }
        Block::CodeBlock(_, code) => code.clone(),
        Block::RawBlock(_, raw) => raw.clone(),
        Block::BlockQuote(children) | Block::Div(_, children) => {
            children.iter().map(stringify_block).collect::<Vec<_>>().join("\n")
        }
        Block::BulletList(items) | Block::OrderedList(_, items) => {
            items
                .iter()
                .map(|item| item.iter().map(stringify_block).collect::<Vec<_>>().join("\n"))
                .collect::<Vec<_>>()
                .join("\n")
        }
        Block::DefinitionList(items) => {
            items
                .iter()
                .map(|(term, definitions)| {
                    format!(
                        "{}\n{}",
                        term.iter().map(stringify_inline).collect::<Vec<_>>().join(""),
                        definitions
                            .iter()
                            .map(|definition|
                                definition
                                    .iter()
                                    .map(stringify_block)
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            )
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
        Block::Header(_, _, children) => {
            children.iter().map(stringify_inline).collect::<Vec<_>>().join("")
        }
        Block::HorizontalRule => "-----\n".to_string(),
        Block::Table(..) | Block::Figure(..) => {
            // Handling for tables and figures would require additional logic
            // to properly format them as text.
            String::new()
        }
        Block::Null => String::new(),
    }
}

pub fn convert_to_text_vec(input_file: &str) -> anyhow::Result<Vec<Vec<String>>> {
    let mut pandoc = pandoc::new();

    let input_file = "src/k8s.md";

    pandoc.add_input(input_file);
    pandoc.set_output_format(pandoc::OutputFormat::Json, vec![]);
    pandoc.set_output(OutputKind::Pipe);
    let pandoc_output = pandoc.execute()?;
    let pandoc_data: Pandoc = match pandoc_output {
        PandocOutput::ToBuffer(content) => Pandoc::from_json(&content),
        _ => panic!("Invalid output"),
    };

    let mut segments: Vec<Vec<String>> = Vec::new();
    let mut current_segment: Vec<String> = Vec::new();

    for block in pandoc_data.blocks.iter() {
        match block {
            Block::Header(_, _, inline) => {
                if !current_segment.is_empty() {
                    segments.push(current_segment);
                }
                current_segment = Vec::new(); // Start a new segment
                let header_text = inline.iter().map(stringify_inline).collect();
                current_segment.push(header_text);
            }
            _ => {
                let content = stringify_block(block);
                if !content.is_empty() {
                    current_segment.push(content);
                }
            }
        }
    }

    if !current_segment.is_empty() {
        segments.push(current_segment);
    }
    // let json_output = serde_json::to_string_pretty(&segments)?;
    Ok(segments)
}
