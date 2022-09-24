use crate::host::Host;

fn print_table(content: &Vec<Vec<String>>) {
    let cols = content[0].len();
    
    // Get column widths
    let mut col_widths: Vec<usize> = vec![0; cols];
    content.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(i, cell)| {
            col_widths[i] = std::cmp::max(col_widths[i], cell.chars().count());
        });
    });

    // Print headers?
    print_separator(&col_widths);
    let headers = vec![String::from("Index"), String::from("Ip Address"), String::from("Hostname")];
    print_row(&headers, &col_widths);

    print_separator(&col_widths);

    // Print rows
    content.iter().for_each(|row| {
        print_row(&row, &col_widths);
    });

    print_separator(&col_widths);

}

fn print_row(row: &Vec<String>, col_widths: &Vec<usize>) {
    let inner = row.iter().enumerate().map(|(i, val)| {
        format!("{:<w$}", val, w=col_widths[i])
    }).collect::<Vec<String>>().join(" | ");
    println!("| {} |", inner);
}

fn print_separator(col_widths: &Vec<usize>) {
    let inner = col_widths.iter().map(|width| format!("-{:-<w$}-", "", w=width)).collect::<Vec<String>>().join("+");
    println!("+{}+", inner);
}

pub fn list_hosts() {
    // Read the hosts file
    // let data = std::fs::read_to_string("./test_host_file").expect("Unable to read file");
    let data = std::fs::read_to_string("C:/Windows/System32/drivers/etc/hosts").expect("Unable to read file");

    // Filter out comments and empty lines
    let lines = data.lines().filter(|l| !l.starts_with("#") && !l.is_empty());

    // Parse each line into a Host struct
    // TODO: Move this code to a parser
    let hosts = lines.enumerate().map(|(i, l)| {
        let parts: Vec<&str> = l.split(" ").collect();
        Host::new(i, parts[0].to_string(), parts[1].to_string())
    });

    // Convert hosts into 2d vector
    let host_vec: Vec<Vec<String>> = hosts.map(|h| {
        h.to_vec()
    }).collect();

    // Print contents
    print_table(&host_vec);
    // println!("+{:-}+{:-^17}+{:-^50}+", "", "");
    // hosts.for_each(|h| println!("| {:<3} | {:<15} | {:<48} |", h.id, h.ip, h.hostname));
    // println!("+{:-^17}+{:-^50}+", "", "");
}
