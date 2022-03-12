extern crate gnuplot;

use std::io::stdin;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//per grafico
use gnuplot::{Figure, Caption, Color, PointSize, PointSymbol, Fix};
use gnuplot::AxesCommon;


//DESCRIZIONE: Stampa un vettore contenente F tipo generico
fn stampa<F>(v: &Vec<F>, n: usize) where F: std::fmt::Display{

    let mut i: usize = 0;
    while i<n{
        print!("|{}", v[i]);
        i = i + 1;
    }
    println!("|");
}

enum RisultatoLetturaValori{
    Good(Vec<Vec<f64>>),
    Bad(Vec<usize>),
}

//DESCRIZIONE: La funzione trova i massimi e i minimi tra i punti inseriti per poter decidere i range rappresentati sugli assi
fn trova_estremi(max: &mut Vec<f64>, min: &mut Vec<f64>, z: & Vec<Vec<f64>>, n: & usize){

    let mut i: usize = 0;
    while i< *n{
        max.push(z[i][0]);
        min.push(z[i][0]);
        i = i +1;
    }
    i = 0;
    while i < *n{
        let mut j: usize = 0;
        while j < z[i].len(){
            if z[i][j] > max[i]{
                max[i] = z[i][j];
            }
            if z[i][j] < min[i]{
                min[i] = z[i][j];
            }
            j = j + 1;
        }
        i = i+1;
    }
    print!("\nI massimi sono:");
    stampa::<f64>(& max, max.len());
    print!("I minimi sono:");
    stampa::<f64>(& min, min.len());
    print!("\n");
}


//La funzione legge una nuova stringa e la stampa prima di restituirla
fn leggi_stringa () -> String {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(n) => {
            println! ("{} bytes read, ALL OK \n", n);
        //    println! ("{}", input);
            }    
        Err(error) => println! ("error: {}", error),
        };
    input
}

//DESCRIZIONE: legge un file di testo in hello.txt
fn leggi_file(p: String) -> String{
    // Create a path to the desired file
    let path = Path::new(&p);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("non e' possibile {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Non e' possibile leggere {}: {}", display,
                                                   why.description()),
        Ok(_) => println!("{} contine:\n{}", display, s),
    }

    s
}

//MODIFICA: Prende una stringa letta, toglie dalla stringa tutti gli spazzi e tutti i caratteri non concessi,verifica che la stringa termini con ; e la ellimina,splitta su i ; e mette le parti in un vettore di stringa. se è conforme (tutte le righe abbiano numero di valori pari alla dimensione), la trasforma in vettore di i8
fn leggi_vettore(mut input: String) -> RisultatoLetturaValori{
    
    //toglie dalla stringa tutti gli spazzi e tutti i caratteri non concessi
    input.retain(|c| c=='0'|| c=='1'||c=='2'||c=='3'||c=='4'||c=='5'||c=='6'||c=='7'|| c=='8'||c=='9'||c==','|| c==';' || c=='.' || c=='-');

    //verifica che la stringa termini con ; e la ellimina
    if input.pop() != Some(';'){
        return RisultatoLetturaValori::Bad(vec![1]);
    }
    //splitta su i ; e mette le parti in un vettore di string
    let v_temp: Vec<&str> = input.split(';').collect();
    let mut v_primario: Vec<Vec<f64>> = vec![];
    let mut i: usize = 0;
    let w_temp: Vec<&str> = v_temp[i].split(',').collect();
    
    let n = w_temp.len();
    let mut j: usize = 0;
    let mut v_secondario: Vec<f64>= vec![];
    while j < w_temp.len(){
            match w_temp[j].parse::<f64>(){
                Ok(a) => {
                    v_secondario.push(a);
                     j = j+1;
                },
            Err(_e) => return RisultatoLetturaValori::Bad(vec![3, i, j]),
        }
    }
    v_primario.push(v_secondario);
    
    i = 1;
    while i < v_temp.len(){
        let w_temp: Vec<&str> = v_temp[i].split(',').collect();
        //Controlla che tutte le righe abbiano numero di valori pari alla dimensione
        if w_temp.len() != n{
            return RisultatoLetturaValori::Bad(vec![2, i]);
        }
        
        let mut j: usize = 0;
        let mut v_secondario: Vec<f64>= vec![];
        while j < w_temp.len(){
            match w_temp[j].parse::<f64>(){
                Ok(a) => {
                    v_secondario.push(a);
                    j = j+1;
                },
                Err(_e) => return RisultatoLetturaValori::Bad(vec![3, i, j]),
            }
        }
        v_primario.push(v_secondario);
        i = i+1;
    }
    
    println!("Sono stati inseriti {} punti", v_primario.len());
    
    let mut cord: Vec<Vec<f64>> = vec![];

    i = 0;
    while i < n {
        let mut j: usize = 0;
        let mut s_temp: Vec<f64> = vec![];
        while j < v_primario.len() {
            s_temp.push(v_primario[j][i]);
            j = j +1;
        }
        cord.push(s_temp);
        i = i+1;
    }
    RisultatoLetturaValori::Good(cord)
}

//DESCRIZIONE: La funzione riconosce gli errori presenti nei punti inseriti e li printa in modo che l'utente possa correggere
fn analisi_errore(s: Vec<usize>){
    print!("Lettura NON andata a buon fine: errore {}, ", s[0]);
    match s[0]{
        1 => println!("l'ultimo carattere del file non è un ; \n"),
        2 => println!("alcuni punti non hanno la dimensione giusta, in particolare il {}\n", s[1]+1),
        3 => println!("nel punto {} l'entrata {} non è corretta\n", s[1]+1, s[2]+1),
        _ => println!("????\n"),
    }

}

// DESCRIZIONE: La funzione legge la risposta dell'utente a una data domanda in cui bisogna fare una scelta resritutisce un booleano
fn leggi_risposta() -> bool{
    let b:bool;
    loop{
        let mut w = String::new();
        stdin().read_line (&mut w)
            .expect("Failed to read line");
        match w.trim(){
            "Si"|"si"|"Yes"|"yes"|"1"|"y"|"Y" => {
                b = true;
                break;
            }
            "No"|"no"|"n"|"N"|"2"|"0" => {
                b = false;
                break;
            }
            _ => println!("Risposta non accettabile, ritenta:"),
        }
    }
    b
}

//DESCRIZIONE: Funzione che presa in input la dimensione e il vettore di vettori z contenente le coordinate, legge nuovi punti e se conformi (se non conforme viene chiesto di scriverlo conforme), li concatena al vettore z (che e' mutabile)
fn legginuovacoordinata(n:&mut usize, z:&mut Vec<Vec<(f64)>>) {
    let nc: Vec<Vec<(f64)>> ;
    loop{
        println! ("Inserire UNA nuova coordinata, cosi fatta 'num,...,num;'");
        let input = leggi_stringa ();
        
        match leggi_vettore(input) {
            RisultatoLetturaValori::Good(a) => {
                nc=a;
                println!("\nIl punto inserito e':");
                let mut i: usize=0;
                while i < *n {
                    stampa::<f64>(& nc[i], nc[i].len());
                    i = i +1;
                }
                break;
            }
            RisultatoLetturaValori::Bad(b) => {
                analisi_errore(b);    
                println! ("Riscrivere la coordinata correttamente\n");
            }    
        };
    }
    let mut i:usize= 0;
    while i < *n {
        z[i].push(nc[i][0]);
        i= i+1;
    }
    
}

//DESCRIZIONE: questa funzione prende in input la dimensione in cui si grafica, il vettore di vettori delle coordinate e grafica, ritorna la figura graficata, (cosi che ogni volta che viene chiamata questa funzione non apre una nuova pagina di plot).
fn grafica (z:& Vec<Vec<(f64)>>, fg: &mut Figure)  {
        
    let mut max: Vec<f64> = vec![];
    let mut min: Vec<f64> = vec![];
    trova_estremi(&mut max, &mut min, & z, & z.len());

    //let mut fg = Figure::new();
    match z.len(){
        1 => {
            let x: Vec<f64> = vec![0f64; z[0].len()];
            fg.axes2d()
            .lines_points(& z[0], & x, &[PointSymbol('O'), PointSize(2.0), Caption("A Plot"), Color("blue")])
            .set_x_label(&"Asse X", &[]).set_y_label(&"Asse Y", &[])
            .set_y_range(Fix(-1f64),Fix(1f64));
        }
        2 => {
            fg.axes2d()
            .lines_points(& z[0], & z[1], &[PointSymbol('O'), PointSize(2.0), Caption("A Plot"), Color("blue")])
            .set_x_label(&"Asse X", &[]).set_y_label(&"Asse Y", &[]);
        }
        3 => {
            fg.axes3d()
            .lines_points(& z[0], & z[1], & z[2], &[PointSymbol('O'), PointSize(2.5), Caption("A line"), Color("blue")])
            .set_x_range(Fix(min[0]),Fix(max[0])).set_y_range(Fix(min[1]),Fix(max[1])).set_z_range(Fix(min[2]),Fix(max[2]))
            .set_x_grid(true).set_y_grid(true).set_z_grid(true)
            .set_x_label(&"Asse X", &[]).set_y_label(&"Asse Y", &[]).set_z_label(&"Asse z", &[]);
        }
        _ => {
            let mut max_sing: f64 = max[1];
            let mut min_sing: f64 = min[1];
            let mut i: usize = 2;
            while i<max.len(){
                if max[i] > max_sing{
                    max_sing = max[i];
                }
                if min[i] < min_sing{
                    min_sing = min[i];
                }
                i = i+1;
            }
        
            let mut i: usize = 1;
            while i < z.len(){
                fg.axes2d()
                .lines_points(& z[0], & z[i], &[PointSymbol('O'), PointSize(2.0), Caption("A Plot"), Color("blue")])
                .set_x_range(Fix(min[0]),Fix(max[0])).set_y_range(Fix(min_sing),Fix(max_sing))
                .set_x_label(&"Asse X", &[]).set_y_label(&"Assi", &[]);
                i = i+1;
            }
        }
    }
    fg.show();
}

//MAIN
fn main() {
    let mut z: Vec<Vec<(f64)>>;
   //non si esce dal loop finche' non viene scritta una riga di punti accettabili e conformi
    loop {
        println! ("Si vogliono leggere delle coordinate da un file (scelta: 1) o inserire manualmente (scelta: 2) ?");
        let risp = leggi_risposta();
        let s: String;
        let mut p: String;
        if risp==true {
            println! ("Scrivi il path al file .csv");
            p = leggi_stringa ();
            p = p.replace("\n", "");
            println! ("Hai inserito {:?}", p);
            s = leggi_file(p);
        } else {
            println! ("\nPrego, inserire i le coordinate cosi fatte: 'num,...,num;num,..,num;");
            s = leggi_stringa();
        }
        
        match leggi_vettore(s){
            RisultatoLetturaValori::Good(a) => {
                z=a;
                println!("I punti sugli assi sono:");
                let mut i: usize=0;
                while i < z.len() {
                    stampa::<f64>(& z[i], z[i].len());
                    i = i +1;
                }
                break;
            }
            RisultatoLetturaValori::Bad(b) => {
                analisi_errore(b);                
                println!("Correggere l'errore prima della risposta alla successiva richiesta in caso di lettura da file, altrimenti correggere \n");
            }
    
        };
    }
    

    let mut fg = Figure::new();
    grafica(& z, &mut fg);

//ulterire loop per l'aggiunta di coordinate e dal quale non si esce finche': o l'utente non vuole inserire coordinate o le coordinate inserire non sono conformi.
    loop{
        println! ("Aggiungere altre coordinate?");
        let risp2= leggi_risposta ();
        if risp2==true {
            legginuovacoordinata(&mut z.len(), &mut z);
            fg.clear_axes();
            grafica(& z, &mut fg);

        } else{
            break;
        }
    }
}

