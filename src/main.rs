use ansi_escapes;
use is_even::IsEven;
use std::io;
fn main() {
    let mut tab = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let players = ["O", "X"];
    let mut turno = 0;
    let mut player = 0;
    loop {
        // Limpa a tela
        print!("{}", ansi_escapes::ClearScreen);
        // exibe o tabuleiro
        println!(
            "{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}\n",
            tab[0], tab[1], tab[2], tab[3], tab[4], tab[5], tab[6], tab[7], tab[8]
        );

        if turno >= 9 {
            println!("empate");
            break;
        } else if turno.is_even() {
            player = 1;
        } else {
            player = 0;
        }
        println!("{}: ", players[player]);
        let mut numero = String::new();
        io::stdin()
            .read_line(&mut numero)
            .expect("falha ao ler linha");
        let numero: usize = numero.trim().parse().expect("não é um numero");

        if tab[numero - 1] != players[1] && tab[numero - 1] != players[0] {
            tab[numero - 1] = players[player];
            if chkw(tab, players[0]) || chkw(tab, players[1]) {
                print!("{}", ansi_escapes::ClearScreen);
                // exibe o tabuleiro
                println!(
                    "{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}\n",
                    tab[0], tab[1], tab[2], tab[3], tab[4], tab[5], tab[6], tab[7], tab[8]
                );
                println!("Player {} ganhou", players[player]);
                break;
            }
            if player == 0 {
                player = 1;
            } else if player == 1 {
                player = 0;
            }

            turno = turno + 1;
        }
    }
}
fn chkw(tab: [&str; 9], player: &str) -> bool {
    let mut win = false;
    if chkv(tab[0], tab[1], tab[2], player)
        || chkv(tab[3], tab[4], tab[5], player)
        || chkv(tab[6], tab[7], tab[8], player)
        || chkv(tab[0], tab[3], tab[6], player)
        || chkv(tab[1], tab[4], tab[7], player)
        || chkv(tab[2], tab[5], tab[8], player)
        || chkv(tab[3], tab[4], tab[5], player)
        || chkv(tab[0], tab[4], tab[8], player)
        || chkv(tab[2], tab[4], tab[6], player)
    {
        win = true
    }
    win
}
fn chkv(v1: &str, v2: &str, v3: &str, player: &str) -> bool {
    let mut chk = false;
    if v1 == v2 && v2 == v3 && v3 == player {
        chk = true;
    }
    chk
}
