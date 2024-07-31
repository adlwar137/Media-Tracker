pub fn parse_text(input: String) -> Vec<[String; 2]> {
    let mut bparse = Vec::new();

    let parts = input.split('\n');
    
    for part in parts {
        if part.is_empty() {
            continue;
        }
        bparse.push(part); 
    }

    let mut aparse = Vec::new();

    for part in bparse {
        aparse.push(parse_line(part.to_string()));
    }

    aparse
}

fn parse_line(input: String) -> [String; 2] {
    //split by colon and remove whitespace
    let sides: Vec<_> = input.split(':').collect();

    //remove whitespace
    let (mut fieldName, mut field) = (sides[0].trim().to_string(), sides[1].trim().to_string());

    //remove first and last (The quotation marks)
    fieldName.pop();
    fieldName.remove(0);

    field.pop();
    field.remove(0);

    //return
    [fieldName, field]
}
