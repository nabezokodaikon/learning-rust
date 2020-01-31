type UserName = String;

#[derive(Debug)]
enum Task {
    Open,
    AssignedTo(UserName),
    Working {
        assignee: UserName,
        remaing_hours: u16,
    },
    Done,
}

use crate::Task::*;

fn main() {
    let tasks = vec![
        AssignedTo(String::from("junko")),
        Working {
            assignee: String::from("hiro"),
            remaing_hours: 18,
        },
        Done,
    ];

    for (i, task) in tasks.iter().enumerate() {
        match task {
            AssignedTo(assignee) => println!("タスク{}は{}にアサインされています", i, assignee),
            Working {
                assignee,
                remaing_hours,
            } => println!(
                "タスク{}は{}さんが作業中です。残り{}時間の見込み",
                i, assignee, remaing_hours
            ),
            _ => println!("タスク{}はその他のステータス({:?})です", i, task),
        }
    }
}
