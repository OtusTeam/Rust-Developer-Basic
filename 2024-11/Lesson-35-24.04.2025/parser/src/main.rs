// Пишем простой парсер. У нас будет следующая архитектура:
// Input - читает входные данные и разбивает их на отдельные токены
// Lexer - принимает токены из Input и разбирает их как отдельные лексемы 
// Parser - принимает лексемы из Lexer и работает с ними (в настоящем компиляторе он строил бы из них синтаксическое дерево)

// В нашем примере мы просто будем читать из входной строки слова. Если слово начинается с '{', то это начало блока и
// следующее слово должно быть концом блока и закрываться '}'. Если слово не начинается с '{', то оно просто добавляется в результат.
// Парсер должен уметь автодополнять блоки, если они не закрыты. Например:
// Входная строка: "{ab aba ba {bb bb} {ab aa"
// Выходная строка: "{ab aba} ba {bb bb} {ab aa}"
// Для этого парсер должен передать команду в Input, чтобы тот дополнил следующее слово символом "}" если нужно.
// Таким образом состояние объекта Input будет меняться и из Parser, и из Lexer.
// Чтобы это работало требуется использовать Interior Mutability,
// чтобы как-то попросить Input автоматически дополнить следующее слово, если требуется.

struct Logger {}

struct ParserMessage {
    autocomplete: String,
}

struct Input<'a> {
    input: String,
    logger: &'a Logger,
}

impl Input<'_> {
    fn read(&mut self) -> String {
        let whitespace = self.input.find(' ').unwrap_or(self.input.len());
        let expression: String = self.input.drain(..whitespace).collect();
        if !self.input.is_empty() {
            self.input.drain(..1);
        }

        // Ваш код сюда. Если Parser попросил автодополнить, то автодополните;
        // if parse дал команду автодополнить {
        //      if !expression.ends_with(autocomplete_string) {
        //          return t + autocomplete_string;
        //      }
        // }
        expression
    }
}

struct Lexer<'a> {
    // не модифицируйте эту структуру
    input: Input<'a>,
}

impl Lexer<'_> {
    fn call(&mut self) -> String {
        let from_input = self.input.read();
        if from_input.starts_with('{') {
            return "block_start:".to_owned() + &from_input;
        }
        if from_input.is_empty() {
            return "end".to_owned();
        }
        from_input
    }
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    logger: &'a Logger,
}

impl Parser<'_> {
    fn parse(&mut self) -> String {
        let mut parsed = vec![];
        let mut value = self.lexer.call();

        while &value != "end" {
            let mut v = value;
            if v.starts_with("block_start:") {
                let fixed_v = v.strip_prefix("block_start:").unwrap();

                // ваш код сюда: передайте команду в Input, чтобы он автодополнил следующее слово "}"

                v = fixed_v.to_owned();
            }
            parsed.push(v);
            value = self.lexer.call();
        }

        parsed.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "{ab aba ba {bb bb} {ab aa".to_owned();
        let expected = "{ab aba} ba {bb bb} {ab aa}".to_owned();

        let logger = &Logger {};
        let mut p = Parser {
            logger,
            lexer: Lexer {
                input: Input { input, logger },
            },
        };
        assert_eq!(p.parse(), expected);
    }
}
