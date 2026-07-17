use petgraph::csr::Csr;


type RTGraph<N, E> = Csr<N, E>;

#[derive(Default)]
pub struct Runtime {
    graph:     RTGraph<Instr, ()>,
    pub heap:      Vec<HeapKind>,
    alloc_ptr: u32,
    node_ptr:  u32,
    scope_st:  Vec<u32>,
    scope_st_ptr: u32,
    instr_st:  Vec<Instr>,
    data_st:   Vec<u32>,  
    pub entry_point: u32,
    run_direction: RTDirection,
}

impl Runtime {
    pub fn new(g: RTGraph<Instr, ()>) -> Self {
        Runtime { graph: g, ..Default::default() }
    }
    pub fn run(&mut self) {

        self.node_ptr = self.entry_point;
        self.scope_st.push(0);
        self.scope_st_ptr = 0;

        self.push_instr(self.graph[self.entry_point].clone());


        while self.instr_st.len() != 0 {
            match self.run_direction {
                RTDirection::Execute => {
                    let instr = self.instr_st.pop().unwrap();
                    instr.exe(self);
                }
                RTDirection::Traverse => {
                    let nbors = self.graph.neighbors_slice(self.node_ptr);
                    
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
    pub fn alloc(&mut self, num: u32) {

        self.alloc_ptr += num;
        for _ in 0..num {
            self.heap.push(HeapKind::None);
        }
    }
    pub fn mem_wr(&mut self, addr: u32, val: HeapKind) {
        self.heap[addr as usize] = val;
    }
    pub fn mem_rd(&self, addr: u32) -> HeapKind {
        self.heap[addr as usize].clone()
    }
    pub fn set_node_ptr(&mut self, node: &u32) {
        self.node_ptr = *node;
        self.push_instr(self.graph[self.node_ptr].clone());
    }
}

#[derive(Clone, Debug)]
pub enum Instr {
    ADD,
    SUB,
    MUL,
    DIV,
    RET{node: u32, path: u32},
    SCP{addr: u32, alloc: u32},
    SPK{n: u32},
    POP,
    PSH{data: u32},
    LOA{addr: u32}, 
    CALL{node: u32},
    IF{cond: u32, t: u32, f: u32},
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
                
                rt.alloc(*alloc);
            },
            Instr::SPK { n } => {
                rt.scope_st_ptr += n;
            },
            Instr::LOA { addr } => {
                let scope_addr = rt.scope_st[rt.scope_st.len() - 1 - rt.scope_st_ptr as usize];
                let heap_data = &rt.heap[(scope_addr + addr) as usize];

                if let HeapKind::Num(n) = heap_data {
                    rt.run_direction = RTDirection::Execute;
                    rt.instr_st.pop();
                    rt.data_st.push(*n);
                }
            },
            Instr::IF {cond, .. } => {
                rt.set_node_ptr(cond);
                rt.run_direction = RTDirection::Traverse;
            },
            _ => ()
        }
    }
    pub fn exe(&self, rt: &mut Runtime) {
        match &self {
            Instr::ADD => {
                let a = unsafe { rt.data_st.pop().unwrap_unchecked() };
                let b = unsafe { rt.data_st.pop().unwrap_unchecked() };

                rt.data_st.push(a + b);
            },
            Instr::SUB => {
                let a = unsafe { rt.data_st.pop().unwrap_unchecked() };
                let b = unsafe { rt.data_st.pop().unwrap_unchecked() };

                rt.data_st.push(a - b);
            },
            Instr::MUL => {
                let a = unsafe { rt.data_st.pop().unwrap_unchecked() };
                let b = unsafe { rt.data_st.pop().unwrap_unchecked() };

                rt.data_st.push(a * b);
            },
            Instr::DIV => {
                let a = unsafe { rt.data_st.pop().unwrap_unchecked() };
                let b = unsafe { rt.data_st.pop().unwrap_unchecked() };

                rt.data_st.push(a / b);
            },
            Instr::RET { node, path } => {
                let nbors = rt.graph.neighbors_slice(*node);
                let nbor = nbors[*path as usize];
                let len = nbors.len();

                rt.run_direction = RTDirection::Traverse;
                rt.set_node_ptr(&nbor);

                if len - 1 < *path as usize {
                    rt.push_instr(Instr::RET { node: rt.node_ptr, path: path + 1 });
                }
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
                let scope_addr = rt.scope_st[rt.scope_st.len() - 1 - rt.scope_st_ptr as usize];
                unsafe { rt.heap[(scope_addr + addr) as usize] = HeapKind::Num(*rt.data_st.last().unwrap_unchecked()); }
            },
            Instr::CALL { node } => {
                rt.run_direction = RTDirection::Traverse;
                rt.set_node_ptr(node);
            },
            Instr::IF {t, f, .. } => {
                let cond = unsafe { rt.data_st.pop().unwrap_unchecked() != 0 };

                rt.run_direction = RTDirection::Traverse;
                rt.set_node_ptr(if cond { t } else { f });
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

#[derive(Default, Clone, Debug)]
pub enum HeapKind {
    #[default]
    None,
    Err,
    Num(u32),
}
