use crate::error::BoxedError;
use crate::file_util::load_file;
use crate::lexical_parsing::simple_line_error::SimpleLineError;
use crate::lexical_parsing::symbols::{
    get_all_symbol, Keyword, Punctuation, Symbol, CHAR_DELIMITER, LIST_SEPARATOR_CHARACTER,
    STRING_DELIMITER,
};
use crate::util::{join_file_name};

pub const COMMENT_CHARACTER: char = '#';
pub const OPEN_BRACKET_CHARACTER: char = '(';
pub const CLOSE_BRACKET_CHARACTER: char = ')';
pub const OPEN_INDEXER_CHARACTER: char = '[';
pub const CLOSE_INDEXER_CHARACTER: char = ']';

/// Takes a line of code and returns an array of symbols
#[allow(clippy::single_match)]
pub fn get_symbols_from_line(line: &str) -> Result<Vec<Symbol>, String> {
    fn process_buffer(buffer: &mut String, symbol_line: &mut Vec<Symbol>) -> Result<(), String> {
        if buffer.is_empty() {
            return Ok(());
        }

        let symbol = get_all_symbol(buffer)?;
        symbol_line.push(symbol);
        buffer.clear();
        Ok(())
    }

    let mut symbol_line = Vec::new();

    let mut buffer = String::new();
    let mut in_string: Option<char> = None; // Option<delimiter>
    let mut bracket_depth = 0;
    let mut indexer_depth = 0;
    let mut next_character_escaped = false;

    for c in line.chars() {
        //? String handling
        if let Some(delimiter) = in_string {
            if next_character_escaped {
                buffer.push(c);
                next_character_escaped = false;
                continue;
            }

            if delimiter == c {
                buffer.push(c);
                in_string = None;
                process_buffer(&mut buffer, &mut symbol_line)?;
                continue;
            }

            buffer.push(c);
            continue;
        } else if STRING_DELIMITER == c || CHAR_DELIMITER == c {
            buffer.push(c);
            in_string = Some(c);
            continue;
        }

        //? Comments
        if c == COMMENT_CHARACTER {
            break;
        }

        if bracket_depth == 0 && indexer_depth == 0 {
            match c {
                //? Process buffer, ignore c
                ' ' => {
                    process_buffer(&mut buffer, &mut symbol_line)?;
                    continue;
                }
                //? Process buffer, then process c
                OPEN_BRACKET_CHARACTER
                | CLOSE_BRACKET_CHARACTER
                | OPEN_INDEXER_CHARACTER
                | CLOSE_INDEXER_CHARACTER
                | LIST_SEPARATOR_CHARACTER => {
                    process_buffer(&mut buffer, &mut symbol_line)?;
                }
                _ => {}
            };
        }

        //? List separator
        // if c == LIST_SEPARATOR_CHARACTER {
        //
        // }

        //? Start bracket
        if c == OPEN_BRACKET_CHARACTER {
            if bracket_depth != 0 {
                buffer.push(c);
            }
            bracket_depth += 1;
            continue;
        }

        //? Start bracket
        if c == OPEN_INDEXER_CHARACTER {
            if indexer_depth != 0 {
                buffer.push(c);
            }
            indexer_depth += 1;
            continue;
        }

        //? End bracket section
        if c == CLOSE_BRACKET_CHARACTER {
            bracket_depth -= 1;

            match bracket_depth {
                0 => {
                    symbol_line.push(get_bracketed_symbols_type(get_symbols_from_line(
                        buffer.as_str(),
                    )?));
                    buffer.clear();
                }
                i32::MIN..=-1 => {
                    return Err(
                        "Closing bracket found with no corresponding opening bracket".to_string(),
                    );
                }
                _ => {
                    buffer.push(c);
                }
            }

            continue;
        }

        //? End indexer
        if c == CLOSE_INDEXER_CHARACTER {
            indexer_depth -= 1;

            match indexer_depth {
                0 => {
                    if symbol_line.is_empty() {
                        return Err("Indexers must be applied to something".to_string());
                    }

                    let applied_to = symbol_line.pop().unwrap();
                    let index = get_symbols_from_line(buffer.as_str())?;

                    symbol_line.push(Symbol::Indexer(Box::new(applied_to), index));

                    buffer.clear();
                }
                i32::MIN..=-1 => {
                    return Err(
                        "Closing indexing bracket found with no corresponding opening bracket"
                            .to_string(),
                    );
                }
                _ => {
                    buffer.push(c);
                }
            }

            continue;
        }

        buffer.push(c);
    }

    if in_string.is_some() {
        return Err("Unclosed string".to_string());
    }

    if bracket_depth != 0 {
        return Err("Unclosed brackets".to_string());
    }

    //? Push remaining data
    if !buffer.is_empty() {
        process_buffer(&mut buffer, &mut symbol_line)?;
    }

    Ok(symbol_line)
}

pub struct Line {
    pub file_name_index: usize,
    pub line_index: usize,
    pub indentation: usize,
    pub symbols: Vec<Symbol>,
}

impl Line {
    pub fn new(
        file_name_index: usize,
        line_index: usize,
        indentation: usize,
        symbols: Vec<Symbol>,
    ) -> Line {
        Line {
            file_name_index,
            line_index,
            indentation,
            symbols,
        }
    }
}

pub struct SymbolData {
    file_names: Vec<String>,
    pub lines: Vec<Line>,
}

impl SymbolData {
    pub fn new() -> SymbolData {
        SymbolData {
            file_names: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_file_name(&mut self, file_name: String) -> usize {
        self.file_names.push(file_name);
        self.file_names.len() - 1
    }

    pub fn add_line(
        &mut self,
        file_name_index: usize,
        line_index: usize,
        indentation: usize,
        line: Vec<Symbol>,
    ) {
        self.lines
            .push(Line::new(file_name_index, line_index, indentation, line))
    }

    pub fn get_error_path(&self, line_index: usize) -> String {
        format!(
            "{} - Line {}",
            self.file_names[self.lines[line_index].file_name_index],
            self.lines[line_index].line_index + 1
        )
    }
}

/// Takes code as an input
///
/// Returns `Vec<indentation, symbol line>`
pub fn convert_to_symbols(file_name: String, symbol_data: &mut SymbolData) -> Result<(), BoxedError> {
    println!("Reading file '{}'", file_name);
    let data = load_file(&file_name)?;

    println!("Processing file '{}'", file_name);
    let file_name_index = symbol_data.add_file_name(file_name.clone());

    for (line_index, line) in data.lines().enumerate() {
        //? Count indentation
        let mut indentation_count: usize = 0;
        let mut indentation_char_count: usize = 0;
        for c in line.chars() {
            if c == ' ' {
                indentation_count += 1
            } else if c == '\t' {
                indentation_count += 4
            } else {
                break;
            }
            indentation_char_count += 1;
        }
        if indentation_count % 4 != 0 {
            return Err(SimpleLineError::new(
                "Indentation must be a multiple of 4 spaces or single tabs".to_string(),
                line_index,
                file_name.clone(),
            ).into());
        }

        //? Get symbols
        let symbols = match get_symbols_from_line(&line[indentation_char_count..]) {
            Err(e) => return Err(SimpleLineError::new(e, line_index, file_name.clone()).into()),
            Ok(symbols) => symbols,
        };

        if !symbols.is_empty() {
            let processed = match &symbols[0] {
                Symbol::Keyword(Keyword::Import) => {
                    if indentation_count != 0 {
                        Err("Import statements cannot be indented".to_string())
                    } else if symbols.len() != 2 {
                        Err("Import statements must be formatted import [file name]".to_string())
                    } else {
                        match &symbols[1] {
                            Symbol::Name(name) => {
                                if name.len() < 2 || name.last().unwrap() != "why" {
                                    Err("File extension must be .why".to_string())
                                } else {
                                    let name = join_file_name(name);

                                    convert_to_symbols(name, symbol_data)?;
                                    Ok(true)
                                }
                            }
                            _ => Err("Import statements must be formatted import [file name]"
                                .to_string()),
                        }
                    }
                }
                _ => Ok(false),
            };

            match processed {
                Ok(true) => continue,
                Ok(false) => {}
                Err(e) => return Err(SimpleLineError::new(e, line_index, file_name.clone()).into()),
            };
        }

        symbol_data.add_line(file_name_index, line_index, indentation_count / 4, symbols);
    }

    println!("Finished processing '{}'", file_name);

    Ok(())
}

fn get_bracketed_symbols_type(symbols: Vec<Symbol>) -> Symbol {
    if symbols.is_empty() {
        return Symbol::List(Vec::new());
    }

    let mut has_separator = false;
    for s in &symbols {
        if matches!(s, Symbol::Punctuation(Punctuation::ListSeparator)) {
            has_separator = true;
            break;
        }
    }

    if !has_separator {
        return Symbol::BracketedSection(symbols);
    }

    let mut list = Vec::new();
    let mut item = Vec::new();

    for s in symbols {
        if matches!(s, Symbol::Punctuation(Punctuation::ListSeparator)) {
            list.push(item);
            item = Vec::new();
        } else {
            item.push(s);
        }
    }

    if !item.is_empty() {
        list.push(item);
    }

    Symbol::List(list)
}
