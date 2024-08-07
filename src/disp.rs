use crate::{config, host::Host, host_file::read_hosts};

fn print_table(content: &Vec<Vec<String>>) {
    if (content.len() == 0) {
        println!("You have no host configurations.");
        return;
    }

    let headers = vec![
        String::from("Id"),
        String::from("Ip Address"),
        String::from("Hostname"),
    ];

    // Calculate column widths
    let mut col_widths: Vec<usize> = vec![2, 10, 8];
    content.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(i, cell)| {
            col_widths[i] = std::cmp::max(col_widths[i], cell.chars().count());
        });
    });

    print_table_top_separator(&col_widths);

    // Print headers
    print_row(&headers, &col_widths);
    print_separator(&col_widths);

    // Print rows
    content.iter().for_each(|row| {
        print_row(&row, &col_widths);
    });

    print_table_bottom_separator(&col_widths);
}

fn print_row(row: &Vec<String>, col_widths: &Vec<usize>) {
    let inner = row
        .iter()
        .enumerate()
        .map(|(i, val)| format!("{:<w$}", val, w = col_widths[i]))
        .collect::<Vec<String>>()
        .join(" │ ");
    println!("│ {} │", inner);
}

fn print_table_top_separator(col_widths: &Vec<usize>) {
    let inner = col_widths
        .iter()
        .map(|width| format!("─{:─<w$}─", "", w = width))
        .collect::<Vec<String>>()
        .join("┬");
    println!("┌{}┐", inner);
}

fn print_table_bottom_separator(col_widths: &Vec<usize>) {
    let inner = col_widths
        .iter()
        .map(|width| format!("─{:─<w$}─", "", w = width))
        .collect::<Vec<String>>()
        .join("┴");
    println!("└{}┘", inner);
}

fn print_separator(col_widths: &Vec<usize>) {
    let inner = col_widths
        .iter()
        .map(|width| format!("─{:─<w$}─", "", w = width))
        .collect::<Vec<String>>()
        .join("┼");
    println!("├{}┤", inner);
}

pub fn list_hosts() {
    let hosts: Vec<Host> = read_hosts();

    // Convert hosts into 2d vector
    let host_vec: Vec<Vec<String>> = hosts.iter().map(|h| h.to_vec()).collect();

    // Print contents
    print_table(&host_vec);
}
