use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::io::prelude::*;
fn main() -> std::io::Result<()>{
    let archivo = File::open("Fuente.jay")?;//abro el archivo
    let reader = BufReader::new(archivo);//creamos reader y pasamos como parametro el archivo
    for (i, line) in reader.lines().enumerate() { //recorro linea por linea del archivo .jay
        let line = line.unwrap();//indicamos si hay error
        println!("la linea {}: {:?}",i+1, line);//mostramos la linea en la que se encuentra y su contenia
        let my_string = String::from(line);//la primera linea la agarro
        let vector1: Vec<&str> = my_string.split("").collect();//agarro caracter por caracter del my_string y lo guardo en un vector
        let mut tokens = HashMap::new();//creamos un hashmap
        tokens.insert(" ",301);tokens.insert("",300);tokens.insert("0",49);tokens.insert("1",50);tokens.insert("2",51);tokens.insert("3",52);tokens.insert("4",53);tokens.insert("5",54);tokens.insert("6",55);tokens.insert("7",56);tokens.insert("8",57);tokens.insert("9",58);tokens.insert("A",65);tokens.insert("B",66);tokens.insert("C",67);tokens.insert("D",68);tokens.insert("E",69);tokens.insert("F",70);tokens.insert("G",71);tokens.insert("H",72);tokens.insert("I",73);tokens.insert("J",74);tokens.insert("K",75);tokens.insert("L",76);tokens.insert("M",77);tokens.insert("N",78);tokens.insert("Ñ",165);tokens.insert("O",79);tokens.insert("P",80);tokens.insert("Q",81);tokens.insert("R",82);tokens.insert("S",83);tokens.insert("T",84);tokens.insert("U",85);tokens.insert("V",86);tokens.insert("W",87);tokens.insert("X",88);tokens.insert("Y",89);tokens.insert("Z",90);tokens.insert("a",97);tokens.insert("b",98);tokens.insert("c",99);tokens.insert("d",100);tokens.insert("e",101);tokens.insert("f",102);tokens.insert("g",103);tokens.insert("h",104);tokens.insert("i",105);tokens.insert("j",106);tokens.insert("k",107);tokens.insert("l",108);tokens.insert("m",109);tokens.insert("n",110); tokens.insert("ñ",164);tokens.insert("o",111);tokens.insert("p",112);tokens.insert("q",113);tokens.insert("r",114);tokens.insert("s",115);tokens.insert("t",116);tokens.insert("u",117);tokens.insert("v",118);tokens.insert("w",119);tokens.insert("x",120);tokens.insert("y",121);tokens.insert("z",122);tokens.insert("(",40);tokens.insert(")",41);tokens.insert("{",123);tokens.insert("}",125);tokens.insert(",",44);tokens.insert(";",59);tokens.insert(".",46);tokens.insert("=",61);tokens.insert("+",43);tokens.insert("-",45);tokens.insert("*",42);tokens.insert("/",47);tokens.insert(">",62);tokens.insert("!",33);tokens.insert("<",60);tokens.insert("&",38);tokens.insert("|",124);tokens.insert("\\",92);//y le insertamos todos los caracteres
        for index in vector1.iter(){//recorro el segundo vector en el cual se encuentra la oracion separada por letras
            if !tokens.contains_key(index){/*con el if lo que hago es que el "index" que esta recorriendo el vector1 lo coloco dentro de contains_keypara comprobar que la palabra se encuentre dentro del mapa tokens*/
                println!("La linea {}: tiene un error lexico, {:?}\n\n",i+1,index);//indicamos la linea y el caracter con error
                panic!("ERROR LEXICO: {:?}",index);
            }
        }
        println!("La linea {}: no tiene error lexico", i+1);//llevamos el contador y mostramos que no se encuentra
    }
    Ok(())
}