fn main() {

   

 let mut database: Vec<Book>=Vec::new();

    register_book("Moby Dick".to_string(), "Dosyoveski".to_string()
    , 1864, Category::Horror, &mut database);
    register_book("Suç ve Ceza".to_string(), "Dosyoveski".to_string(), 1934
    , Category::Criminal, &mut database);

   
    get_books_by_writer("Dosyoveski".to_string(), &mut database);
    get_by_year(1954, &mut database);
   
}


#[derive(Debug,Clone)]

struct Movie {
    name:String,
    director:String,
    realese_year:u32,
    category:Category
}



#[derive(Debug,Clone)]
struct Book {

    name:String,
    writer:String,
    publish_year:u32,
    category:Category
}

#[derive(Debug,Clone)]

enum Category {
    Horror,
    Scince,
    Action,
    Adventure,
    Criminal
}

fn register_book(name:String,writer:String,publish_year:u32,
    category:Category,database:&mut Vec<Book>) {
        if name.is_empty() {

            println!("Kitap ismi  boş bırakılamaz")
            
        }else {
            let book = Book{
                name:name,
                writer:writer,
                publish_year:publish_year,
                category:category
            };
            database.push(book.clone());
            
        }
}

fn get_by_name(name:String,database:&mut Vec<Book>) {
    for data in database  {

        if name == data.name {

            println!("{} isimli kitabın bilgileri {:?}",name,data)
            
        }
        
    }
}

fn get_books_by_writer(writer:String,database:&mut Vec<Book>) {
    for data in database  {
        if writer == data.writer {
            println!("{} bu yazarın kitap bilgileri {:?} ",writer ,data)
            
        }


        
    }
    
}

fn get_by_year(publish_year:u32,database:&mut Vec<Book>) {
    for data in database  {

        if publish_year <1960  {

            println!("{} Bu yıldan önce çıkan kitaplar {:?}",publish_year,data.name)
            
        }
        
    } 


}
