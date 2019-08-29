// Tell the compiler to not holler at me for using snake case
#![allow(non_snake_case)]


use std::fs::File;

// Import the Grammar.rs file
mod Grammar;

// We have to use a "crate" in order to get regex functionality. 
// Crates are libraries that aren't part of the standard library
extern crate regex;

// We'll also be using the RNG crate for random number functionality
extern crate rand;
use rand::Rng;


fn main() 
{
    let mut symbolsToParse: Vec<Grammar::Symbol> = Vec::new();
    let mut currentSymbol: Grammar::Symbol;
    let mut rand = rand::thread_rng();
    let mut randProduction: usize;

    // Get the start symbol of this grammar
    let mut grammarFile = File::open(std::env::args().nth(1).unwrap())
                            .unwrap();
    let mut grammar = Grammar::new(grammarFile);
    let startSymbol = grammar.getStartSymbol();

    // Create another instance of the grammar file and the grammar... 
    // Because Rust
    grammarFile = File::open(std::env::args().nth(1).unwrap()).unwrap();       
    grammar = Grammar::new(grammarFile);
    let symbolTable = grammar.getSymbolTable();

    // Push the start symbol onto the stack
    symbolsToParse.push(startSymbol);

    // Continue to generate a sentence until there aren't any symbols left to 
    // parse
    while !symbolsToParse.is_empty() 
    {
        // Get the next symbol to parse
        currentSymbol = symbolsToParse.pop().unwrap();

        // If the symbol is terminal, then print it
        if currentSymbol.isTerminal()
        {
            print!("{} ", currentSymbol.getSymbol());
        }

        // Otherwise, choose a random production of this symbol
        else 
        {
            randProduction = rand.gen_range(0, symbolTable.get(&currentSymbol)
                .unwrap().len());

            // Iterate over all the symbols in the production, but do so in 
            // reverse order in order to maintain the ordering of the symbols 
            // when they're pushed onto the stack.
            for i in (0..symbolTable.get(&currentSymbol).unwrap()
                .get(randProduction).unwrap().len()).rev()
            {
                symbolsToParse.push(symbolTable.get(&currentSymbol).unwrap()
                    .get(randProduction).unwrap().get(i).unwrap().clone());
            }
        }
    }
}
