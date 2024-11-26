use std::sync::Arc;

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tokio::{
    runtime::Runtime,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::{parse_instruction, parse_program};

async fn run_program(
    program: &mut [isize],
    sender: Sender<isize>,
    mut receiver: Receiver<isize>,
) -> Vec<isize> {
    let mut index = 0;
    let mut output = Vec::new();
    while index < program.len() {
        // thread::sleep(Duration::from_millis(100));
        let operand = *program.get(index).unwrap();
        // println!("Index: {index}, Operand: {operand}");
        let (operand, param1, param2, _param3) = parse_instruction(operand);

        if operand == 99 {
            break;
        }

        let pos1 = *program.get(index + 1).unwrap();
        let value1 = if param1 || operand == 3 {
            pos1
        } else {
            *program.get::<usize>(pos1.try_into().unwrap()).unwrap()
        };
        match operand {
            1 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 {
                    pos2
                } else {
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                let dest = *program.get(index + 3).unwrap();
                // println!("dest: {dest} = {value1} + {value2}");
                *program.get_mut::<usize>(dest.try_into().unwrap()).unwrap() = value1 + value2;
                index += 4;
            }
            2 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 {
                    pos2
                } else {
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                let dest = *program.get(index + 3).unwrap();
                // println!("dest: {dest} = {value1} * {value2}");
                *program.get_mut::<usize>(dest.try_into().unwrap()).unwrap() = value1 * value2;
                index += 4;
            }
            3 => {
                let input_value = receiver.recv().await.unwrap();
                // println!("dest: {value1} = {input_value}");
                *program
                    .get_mut::<usize>(value1.try_into().unwrap())
                    .unwrap() = input_value;
                index += 2;
            }
            4 => {
                // println!("output: {value1}");
                output.push(value1);
                let _ = sender.send(value1).await;
                index += 2;
            }
            5 => {
                if value1 != 0 {
                    let pos2 = *program.get(index + 2).unwrap();
                    let value2 = if param2 {
                        pos2
                    } else {
                        *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                    };
                    // println!("Index: {value2}");
                    index = value2.try_into().unwrap();
                } else {
                    index += 3;
                }
            }
            6 => {
                if value1 == 0 {
                    let pos2 = *program.get(index + 2).unwrap();
                    let value2 = if param2 {
                        pos2
                    } else {
                        *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                    };
                    // println!("Index = from: {pos2} = {value2}");
                    index = value2.try_into().unwrap();
                } else {
                    index += 3;
                }
            }
            7 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 {
                    pos2
                } else {
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                let dest = *program.get(index + 3).unwrap();
                let store = if value1 < value2 { 1 } else { 0 };
                // println!("dest: {dest} = {store}");
                *program.get_mut::<usize>(dest.try_into().unwrap()).unwrap() = store;
                index += 4;
            }
            8 => {
                let pos2 = *program.get(index + 2).unwrap();
                let value2 = if param2 {
                    pos2
                } else {
                    *program.get::<usize>(pos2.try_into().unwrap()).unwrap()
                };
                let dest = *program.get(index + 3).unwrap();
                let store = if value1 == value2 { 1 } else { 0 };
                // println!("dest: {dest} = {store}");
                *program.get_mut::<usize>(dest.try_into().unwrap()).unwrap() = store;
                index += 4;
            }
            99 => {
                // We cover this case above
                unreachable!()
            }
            _ => panic!("Invalid operand: {operand}"),
        }
    }
    output
}

fn calculate(
    program: &[isize],
    input: (isize, isize, isize, isize, isize),
    rt: Arc<Runtime>,
) -> isize {
    let use_program = program.to_owned();
    let (first_send, first_recv) = mpsc::channel::<isize>(50);
    let (second_send, second_recv) = mpsc::channel::<isize>(50);
    let (third_send, third_recv) = mpsc::channel::<isize>(50);
    let (forth_send, forth_recv) = mpsc::channel::<isize>(50);
    let (fifth_send, fifth_recv) = mpsc::channel::<isize>(50);
    rt.block_on(async {
        let _ = first_send.send(input.0).await;
        let _ = first_send.send(0).await;
    });
    let first = rt.spawn(async move {
        let mut program = use_program;
        run_program(&mut program, first_send, fifth_recv).await
    });
    let use_program = program.to_owned();
    rt.block_on(async {
        let _ = second_send.send(input.1).await;
    });
    let second = rt.spawn(async move {
        let mut program = use_program;
        run_program(&mut program, second_send, first_recv).await
    });
    let use_program = program.to_owned();
    rt.block_on(async {
        let _ = third_send.send(input.2).await;
    });
    let third = rt.spawn(async move {
        let mut program = use_program;
        run_program(&mut program, third_send, second_recv).await
    });
    let use_program = program.to_owned();
    rt.block_on(async {
        let _ = forth_send.send(input.3).await;
    });
    let forth = rt.spawn(async move {
        let mut program = use_program;
        run_program(&mut program, forth_send, third_recv).await
    });
    let use_program = program.to_owned();
    rt.block_on(async {
        let _ = fifth_send.send(input.4).await;
    });
    let fifth = rt.spawn(async move {
        let mut program = use_program;
        run_program(&mut program, fifth_send, forth_recv).await
    });

    let output = rt.block_on(async {
        let first = first.await.unwrap();
        let _second = second.await.unwrap();
        let _third = third.await.unwrap();
        let _forth = forth.await.unwrap();
        let _fifth = fifth.await.unwrap();
        // println!("1:{first:?}, 2:{second:?}, 3:{third:?}, 4:{forth:?}, 5:{fifth:?}");
        first
    });

    // println!("{output:?}");
    *output.last().unwrap()
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let program = parse_program(input);
    let iter: Vec<(isize, isize, isize, isize, isize)> = (5..=9)
        .flat_map(|first| {
            (5..=9)
                .filter(move |second| *second != first)
                .flat_map(move |second| {
                    (5..=9)
                        .filter(move |third| *third != first && *third != second)
                        .flat_map(move |third| {
                            (5..=9)
                                .filter(move |forth| {
                                    *forth != first && *forth != second && *forth != third
                                })
                                .flat_map(move |forth| {
                                    (5..=9)
                                        .filter(move |fifth| {
                                            *fifth != first
                                                && *fifth != second
                                                && *fifth != third
                                                && *fifth != forth
                                        })
                                        .map(move |fifth| (first, second, third, forth, fifth))
                                })
                        })
                })
        })
        .collect_vec();

    let rt = Arc::new(tokio::runtime::Builder::new_multi_thread().build().unwrap());
    let result = iter
        .par_iter()
        .map(|input| (calculate(&program, *input, rt.clone()), input))
        .max_by_key(|(num, _input)| *num)
        .unwrap();
    // println!("{result:?}");
    result.0.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", (4, 3, 2, 1, 0), 43210)]
    #[case("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", (0, 1, 2, 3, 4), 54321)]
    #[case("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", (1, 0, 4, 3, 2), 65210)]
    #[case("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", (9, 8, 7, 6, 5), 139629729)]
    #[case("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", (9, 7, 8, 5, 6), 18216)]
    fn test_calculate(
        #[case] program: &str,
        #[case] input: (isize, isize, isize, isize, isize),
        #[case] result: isize,
    ) {
        assert_eq!(
            result,
            calculate(
                &parse_program(program),
                input,
                Arc::new(
                    tokio::runtime::Builder::new_current_thread()
                        .build()
                        .unwrap()
                )
            )
        );
    }

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        assert_eq!("89603079", process(input));
    }
}
