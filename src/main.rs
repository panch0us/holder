use std::io;
use rusqlite::{params, Connection, Result};
use chrono::{Datelike, DateTime, Local, NaiveDate};

#[derive(Debug)]
struct Property {
    id: i32,
    title: String,
    kind: String,
    serial_num: String,
    receipt_date: String, // Дата получения имущества (вводится вручную)
}
/*
struct Employee {
    surname: String,
    name: String,
}

struct PropertyKind {
    title: String,
    note: String,
}
*/

fn create_db() -> Result<()> {
    let path = "holder.db3";
    let conn = Connection::open(path).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS property (
            id    INTEGER PRIMARY KEY,
            title  TEXT NOT NULL,
            kind  TEXT NOT NULL,
            serial_num  TEXT NOT NULL,
            receipt_date TEXT NOT NULL
        )",
        (),
    )?;
    Ok(())
}

fn add_property_in_db(property: Property) -> Result<()> {
    let path = "holder.db3";
    let conn = Connection::open(path).unwrap();

    let property = property;
    conn.execute(
        "INSERT INTO property (title, kind, serial_num, receipt_date) VALUES (?1, ?2, ?3, ?4)",
        (&property.title, &property.kind, &property.serial_num, &property.receipt_date),
    )?;
    Ok(())
}

fn select_all_property() -> Result<()> {
    let path = "holder.db3";
    let conn = Connection::open(path).unwrap();

    let mut stmt = conn.prepare("SELECT id, title, kind, serial_num, receipt_date FROM property")?;

    let property_iter = stmt.query_map([], |row| {
        Ok(Property {
            id: row.get(0)?,
            title: row.get(1)?,
            kind: row.get(2)?,
            serial_num: row.get(3)?,
            receipt_date: row.get(4)?,
        })
    })?;

    for property in property_iter {
        println!("Found property {:?}", property.unwrap());
    }
    Ok(())
}

fn add_property() -> Property {
    // Данные об имуществе для внесения в БД
    let mut title               = String::new();
    let mut kind                = String::new();
    let mut serial_num          = String::new();
    let mut date_question       = String::new();
    let mut year                = String::new();
    let mut month               = String::new();
    let mut day                 = String::new();
    let date_current: DateTime<Local> = Local::now();
    let mut receipt_date     = NaiveDate::from_ymd_opt(
        date_current.year(),
        date_current.month(),
        date_current.day())
        .unwrap().to_string();

    println!("Введите название имущества: ");
    io::stdin()
        .read_line(&mut title)
        .expect("Ошибка ввода на этапе read_line");

    let title: String = match title.trim().parse() {
        Ok(ok) => ok,
        Err(error) => panic!("Ошибка ввода обработки названия имущества: {:?}", error),
    };

    println!("Введите название вида имущества: ");
    io::stdin()
        .read_line(&mut kind)
        .expect("Ошибка ввода на этапе read_line");

    let kind: String = match kind.trim().parse() {
        Ok(ok) => ok,
        Err(error) => panic!("Ошибка ввода обработки названия вида имущества: {:?}", error),
    };

    println!("Введите название серийного номера имущества: ");
    io::stdin()
        .read_line(&mut serial_num)
        .expect("Ошибка ввода на этапе read_line");


    let serial_num: String = match serial_num.trim().parse() {
        Ok(ok) => ok,
        Err(error) => panic!("Ошибка ввода обработки названия сер. номера: {:?}", error),
    };

    if date_question == String::from("yes") {
        println!("ВЕРНО!");
    }
    println!("Ввести текущую дату добавления имущества в базу? (y/n?)");
    io::stdin()
        .read_line(&mut date_question)
        .expect("Ошибка ввода на этапе read_line");

    date_question = match date_question.trim().parse() {
        Ok(ok) => ok,
        Err(error) => panic!("Ошибка ввода обработки вопроса о дате: {:?}", error),
    };

    if date_question == "n".to_string() || date_question == "н".to_string() {
        println!("Введите год добавления имущества в БД: (формат: 0000)");
        io::stdin()
            .read_line(&mut year)
            .expect("Ошибка ввода на этапе read_line");

        println!("Введите месяц добавления имущества в БД : (формат: 00): ");
        io::stdin()
            .read_line(&mut month)
            .expect("Ошибка ввода на этапе read_line");

        println!("Введите день добавления имущества в БД : (формат: 00): ");
        io::stdin()
            .read_line(&mut day)
            .expect("Ошибка ввода на этапе read_line");

        let year: i32 = match year.trim().parse() {
            Ok(ok) => ok,
            Err(error) => panic!("Ошибка ввода обработки года: {:?}", error),
        };

        let month: u32 = match month.trim().parse() {
            Ok(ok) => ok,
            Err(error) => panic!("Ошибка ввода обработки месяца: {:?}", error),
        };

        let day: u32 = match day.trim().parse() {
            Ok(ok) => ok,
            Err(error) => panic!("Ошибка ввода обработки дня: {:?}", error),
        };

        receipt_date = NaiveDate::from_ymd_opt(
            year,
            month,
            day)
            .unwrap().to_string();

    } else if date_question == "y".to_string() || date_question == "д".to_string() {
        println!("Дата {} введена автоматически", receipt_date);
    } else {
        println!("Указано неверное значение, поэтому дата введена автоматически!");
    }

    let property = Property {
        id: 0,
        title: title,
        kind: kind,
        serial_num: serial_num,
        receipt_date: receipt_date,
    };

    return property
}


fn main() {
    let property = add_property(); // создаем новое имущество
    create_db(); // создаем (открываем) БД
    add_property_in_db(property); // добавдяем имущество в БД
    select_all_property(); // делаем выборку имущества из БД
}