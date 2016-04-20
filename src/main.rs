extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::thread::spawn;
use std::sync::mpsc::channel;

const NUM_SAME: usize = 100;
const NUM_THS: usize = 10;
const BATCH_SIZE: usize = 10000;
const MAX_TRY: usize = 1000000000;

fn main() {
    let mut txs = Vec::new();
    let mut rxs = Vec::new();
    let mut table = [0;NUM_SAME+1];
    let mut tries = 1;

    for _ in 0..NUM_THS {
        let (com_tx, rx) = channel();
        let (tx, res_rx) = channel();
        txs.push(com_tx);
        rxs.push(res_rx);
        spawn(move||{
            let mut rng = thread_rng();
            loop {
                match rx.recv() {
                    Err(_) => return,
                    Ok((0, _)) => return,
                    Ok((tries, mut v)) => {
                        let mut v: Vec<usize> = v;
                        'v: for elm in v.iter_mut() {
                            let base = rng.gen::<usize>() % 6;
                            for i in 0..NUM_SAME {
                                let eye = rng.gen::<usize>() % 6;
                                if eye != base {
                                    *elm = i;
                                    break 'v;
                                }
                            }
                            *elm = NUM_SAME;
                        }
                        let _ = tx.send((tries, v));
                    }
                }
            }
        });
    }
    for i in 0..NUM_THS {
        let mut v = Vec::new();
        v.extend_from_slice(&[0;BATCH_SIZE]);
        txs[i].send((tries, v)).unwrap();
        tries += BATCH_SIZE;
    }
    'outer2: for _ in 0..(MAX_TRY/(NUM_THS*BATCH_SIZE)) {
        for i in 0..NUM_THS {
            let (ret_tries, res) = rxs[i].recv().unwrap();
            for (j, n) in res.iter().enumerate() {
                table[*n] += 1;
                if *n == NUM_SAME {
                    println!("suceeded after {} tries", ret_tries + j);
                    break 'outer2;
                }
            }
            txs[i].send((tries, res)).unwrap();
            tries += BATCH_SIZE;
        }
    }
    for i in 0..(NUM_SAME+1) {
        println!("{}:{}", i, table[i]);
    }
    for i in 0..NUM_THS {
        txs[i].send((0, Vec::new())).unwrap();
    }
}
