pub mod config;
pub mod system;
mod table;
mod brew;
use brew::Brew;
use std::thread;

const FORMULAE: &str = "--formulae";
const CASKS:    &str = "--casks";

/// - Prints new formulae, new casks, and outdated with descriptions
pub fn print_output_with_new_item_desc() {
    let update = Brew::cmd(&["update"]);
    let output = update.stderr(); // `brew update` outputs to stderr

    let outdated_handle = thread::spawn(print_outdated_with_desc);

    if update.stdout.contains("Already up-to-date.\n") {
        
        println!("Already up-to-date.\n");
        
    } else {
        
        thread::scope(|s| {
            let formulae_handle = s.spawn(|| {
                print_new_items(&output, FORMULAE);
            });

            let cask_handle = s.spawn(|| {
                print_new_items(&output, CASKS);
            });

            formulae_handle.join().unwrap();
            cask_handle.join().unwrap();
        });
        
    };

    outdated_handle.join().unwrap();
}

/// Lists all installed items with description
pub fn print_desc_for_all_installed() {
    let handles = [FORMULAE, CASKS]
        .into_iter()
        .map(|item_type| {
            thread::spawn(|| {
                println!(
                    "==> All {}\n{}\n",
                    item_type_display_name(item_type),
                    table::from_columns(
                        Brew::list_with_desc(&["list", "-1", item_type], item_type).array()
                    )
                );
            })
        })
        .collect::<Vec<_>>();
    handles.into_iter().for_each(|h| h.join().unwrap());
}

/// Lists all manually installed formulae with descriptions
pub fn print_desc_for_leaves() {
    println!(
        "==> Leaves\n{}\n",
        table::from_columns(
            Brew::list_with_desc(&["leaves"], FORMULAE).array()
        )
    );
}

/// Prints a table of |name|version|desc| for outdated formulae
fn print_outdated_with_desc() {
    let outdated = Brew::outdated();
    if outdated.stdout.is_empty() { return };
    
    let (items, versions) = outdated.cols();
    
    println!(
        "==> Outdated\n{}\n",
        table::from_columns(
            [
                &items,
                &versions,
                &Brew::cmd_with_items("desc", &items, "--eval-all").cols().1,
            ]
        )
    );
}

/// Prints new items if found in output
fn print_new_items(output: &str, item_type: &str) {
    let search_str = format!("New {}\n", item_type_display_name(item_type));
    
    if let Some(new_items) = extract_new_items(output, &search_str) {
        println!(
            "==> {}{}\n",
            search_str,
            table::from_columns(
                Brew::name_desc_homepage_array(&new_items)
            )
        );
    }
}

fn item_type_display_name(item_type: &str) -> &str {
    match item_type {
        FORMULAE  => "Formulae",
        CASKS     => "Casks",
        _         => panic!("illegal argument: ({item_type})")
    }
}

/// Isolates item list between two string markers
fn extract_new_items<'a>(text: &'a str, search_str: &str) -> Option<Vec<&'a str>> {
    Some(
        text
            .split_once(search_str)?.1
            .split_once("\n==>")?.0
            .lines()
            .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::extract_new_items;

    #[test]
    fn processing_test() {
        let output = "homebrew/cask).\n==> New Formulae\nform1\nform2\n==> New Casks\ncask1\ncask2\n==> Outdated Formulae\naugeas\nawscli\ncjson\ncmake\ndav1d\n";

        let new_items = extract_new_items(&output, "New Formulae\n").unwrap();
        assert_eq!(new_items, vec!["form1", "form2"]);

        let new_items = extract_new_items(&output, "New Casks\n").unwrap();
        assert_eq!(new_items, vec!["cask1", "cask2"]);
    }
}
