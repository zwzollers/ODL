use std::{collections::HashMap, default};

use petgraph::graph::NodeIndex;
use petgraph::{data::DataMapMut, graph::DiGraph};
use petgraph::visit::NodeIndexable;

use crate::ast::{self, AST, Module, Statement, TopStructre};


type RTGraph<N, E> = DiGraph<N, E, u32>;

#[derive(Default)]
pub struct Runtime {
    graph:     RTGraph<Instr, ()>,
    heap:      Vec<HeapKind>,
    alloc_ptr: u32,
    node_ptr:  NodeIndex,
    scope_st:  Vec<u32>,
    scope_st_ptr: u32,
    instr_st:  Vec<Instr>,
    data_st:   Vec<u32>,  
    entry_point: u32,
    run_direction: RTDirection,
}

impl Runtime {
    pub fn run(&mut self) {
        let start_node = self.graph.from_index(self.entry_point as usize);

        self.instr_st.push(self.graph[start_node].clone());
        self.node_ptr = start_node;

        while self.instr_st.len() != 0 {
            match self.run_direction {
                RTDirection::Execute => {
                    let instr = self.instr_st.pop().unwrap();
                    instr.exe(self);
                }
                RTDirection::Traverse => {
                    let nbors: Vec<NodeIndex> = self.graph.neighbors_directed(self.node_ptr, petgraph::Direction::Outgoing).collect();
                    
                    if nbors.len() == 0 {
                        self.run_direction = RTDirection::Execute;
                    } else {
                        self.node_ptr = nbors[0];
                        self.push_instr(self.graph[self.node_ptr].clone());
                    }
                }
            }
        }
    } 
    pub fn push_instr(&mut self, instr: Instr) {
        self.instr_st.push(instr.clone());
        instr.add(self);

    }
}

#[derive(Clone)]
enum Instr {
    ADD,
    SUB,
    MUL,
    DIV,
    RET{node: NodeIndex, path: u32},
    SCP{addr: u32, alloc: u32},
    SPK{n: u32},
    POP,
    PSH{data: u32},
    LOA{addr: u32}, 
    CALL{node: NodeIndex},
    IF{cond: NodeIndex, t: NodeIndex, f: NodeIndex},
}

impl Instr {
    pub fn add(&self, rt: &mut Runtime) {
        match &self {
            Instr::ADD | Instr::SUB | Instr::MUL | Instr::DIV => { 
                rt.push_instr(Instr::RET { node: rt.node_ptr, path: 1 });
            }
            Instr::SCP { addr, alloc } => {
                let heap_addr = (rt.scope_st.last().unwrap() + addr) as usize;
                rt.heap[heap_addr] = HeapKind::Num(rt.alloc_ptr);
                rt.scope_st.push(rt.alloc_ptr);
                rt.alloc_ptr += alloc;

                for _ in 0..*alloc {
                    rt.heap.push(HeapKind::None);
                }
            },
            Instr::SPK { n } => {
                rt.scope_st_ptr += n;
            },
            Instr::LOA { addr } => {
                let scope_addr = rt.scope_st[rt.scope_st.len() - rt.scope_st_ptr as usize];
                let heap_data = &rt.heap[(scope_addr + addr) as usize];

                if let HeapKind::Num(n) = heap_data {
                    rt.run_direction = RTDirection::Execute;
                    rt.instr_st.pop();
                    rt.data_st.push(*n);
                }
            },
            Instr::IF {cond, .. } => {
                rt.node_ptr = *cond;
            },
            _ => ()
        }
    }
    pub fn exe(&self, rt: &mut Runtime) {
                match &self {
            Instr::ADD => {
                let a = rt.data_st.pop().unwrap();
                let b = rt.data_st.pop().unwrap();

                rt.data_st.push(a + b);
            },
            Instr::SUB => {
                let a = rt.data_st.pop().unwrap();
                let b = rt.data_st.pop().unwrap();

                rt.data_st.push(a - b);
            },
            Instr::MUL => {
                let a = rt.data_st.pop().unwrap();
                let b = rt.data_st.pop().unwrap();

                rt.data_st.push(a * b);
            },
            Instr::DIV => {
                let a = rt.data_st.pop().unwrap();
                let b = rt.data_st.pop().unwrap();

                rt.data_st.push(a / b);
            },
            Instr::RET { node, path } => {
                
            },
            Instr::SCP { .. } => {
                rt.scope_st.pop();
            },
            Instr::SPK { n } => {
                rt.scope_st_ptr -= n;
            },
            Instr::POP => {
                rt.data_st.pop();
            },
            Instr::PSH { data } => {
                rt.data_st.push(*data);
            },
            Instr::LOA { addr } => {
                let scope_addr = rt.scope_st[rt.scope_st.len() - rt.scope_st_ptr as usize];
                rt.heap[(scope_addr + addr) as usize] = HeapKind::Num(*rt.data_st.last().unwrap());
            },
            Instr::CALL { node } => {
                rt.node_ptr = *node;
            },
            Instr::IF {t, f, .. } => {
                let cond = rt.data_st.pop().unwrap() != 0;

                rt.run_direction = RTDirection::Traverse;
                rt.node_ptr = if cond { *t } else { *f };
            },
        }
    }
}

#[derive(Default)]

enum RTDirection {
    Execute,
    #[default]
    Traverse,
}

#[derive(Default)]
enum HeapKind {
    #[default]
    None,
    Err,
    Num(u32),
}
