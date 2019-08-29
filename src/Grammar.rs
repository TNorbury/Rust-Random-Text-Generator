// Tell the compiler to not holler at me for not using snake case
#![allow(non_snake_case)]

use std::collections::HashMap;
use std::hash::{Hash, Hasher, SipHasher};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

// We have to use a "crate" in order to get regex functionality. 
// Crates are libraries that aren't part of the standard library
extern crate regex;
use regex::Regex;

const START_TOKEN: &'static str = "{";
const END_TOKEN: &'static str = "}";
const BODY_END: &'static str = ";";

// A Struct which represents a complete Grammar
pub struct Grammar 
{
    _startSymbol: Symbol,
    _symbolTable: HashMap<Symbol, Vec<Vec<Symbol>>>,
}

/**
 * Creates a new Gramamr.
 * Algorithm adapted from Dr. Hansen's Grammar.java code.
 */
pub fn new(fileToRead: File) -> Grammar
{
    let mut firstSymbolRead = true;

    // Define the variables that will be used to create the grammar struct.
    let mut symbolTable: HashMap<Symbol, Vec<Vec<Symbol>>> = HashMap::new();
    let mut startSymbol: Symbol = newSymbol("".to_string()); 

    let mut symbolList: Vec<Symbol> = Vec::new();

    // Boolean flags to keep track of what state we're in.
    let mut readingAProduction = false;
    let mut readingBodies = false;

    // Used during parsing to represent parts of productions
    let mut leftHandSideSymbol: Symbol = newSymbol("".to_string());

    // Create a reader for the file
    let fileReader = BufReader::new(fileToRead);

    // Iterate through the lines in the file
    for lines in fileReader.lines()
    {
        for token in lines.unwrap().split(" ")
        {
            // Take the token and convert it into a String reference
            let currentSymbol: String = token.to_string();
            
            // If we're not reading a production, check to see if the current 
            // symbol is the production start symbol
            if readingAProduction == false
            {
                readingAProduction = currentSymbol == START_TOKEN;
            }

            // Otherwise, see if we've reached the production end token. If so
            // then set the appropriate flags
            else if currentSymbol == END_TOKEN
            {
                readingAProduction = false;
                readingBodies = false;
            }

            // Otherwise, we know that we're reading a production
            else
            {

                // If we're not reading a body then we're reading a new 
                // production
                if !readingBodies
                {
                    // Add this new left hand side symbol to the symbol table
                    leftHandSideSymbol = newSymbol(currentSymbol.to_string());
                    symbolTable.insert(leftHandSideSymbol, Vec::new());

                    // Rust doesn't allow something to be added to a collection
                    // and then used afterwards unless it implements the copy
                    // trait. However, because Symbol contains a String, which
                    // can't be copied, symbol can't implement copy. As such we
                    // have to create another instance of symbol
                    leftHandSideSymbol = newSymbol(currentSymbol.to_string());

                    // We are now reading production bodies 
                    readingBodies = true;

                    // If this is the first symbol we've read, then that means 
                    // it's the start symbol for this grammar
                    if firstSymbolRead == true
                    {
                        firstSymbolRead = false;
                        startSymbol = newSymbol(leftHandSideSymbol
                            .getSymbol());
                    }
                }

                // Otherwise, we're reading the body of a production
                else
                {
                    // If the symbol is the end symbol, then we're done reading a 
                    // production
                    if currentSymbol == BODY_END
                    {
                        // Since we're at the end of this body, add the symbol
                        // list to the symbol table
                        symbolTable.get_mut(&leftHandSideSymbol).unwrap()
                            .push(symbolList);

                        // Create a new vec for the symbol list
                        symbolList = Vec::new();
                    }

                    // Otherwise, we're not at the end, so add this symbol to the 
                    // current symbol list.
                    else
                    {
                        // If the current symbol is a new line character, then
                        // create a symbol with a literal "\n" in order to 
                        // create an actual newline
                        if currentSymbol == "\\n"
                        {
                            symbolList.push(newSymbol("\n".to_string()));
                        }
                        else
                        {
                            symbolList.push(newSymbol(currentSymbol.trim()
                                .to_string()));
                        }
                    }
                }
            } // else
        }
    } // for


    let newGrammar = 
        Grammar {_startSymbol: startSymbol, _symbolTable: symbolTable};
    return newGrammar;
}


// Implement functionality for the grammar
impl Grammar
{
    /**
     * @return The grammar's symbol table
     */
    pub fn getSymbolTable(self) -> HashMap<Symbol, Vec<Vec<Symbol>>>
    {
        return self._symbolTable;
    }

    /**
     * @return The grammar's start symbol
     */
    pub fn getStartSymbol(self) -> Symbol
    {
        return self._startSymbol;
    }
}

// We want to be able to clone Symbols (however, we can't make copies of them)
#[derive(Clone)]
pub struct Symbol
{
    _value: String,
}


/** 
 * This is the contructor for a Symbol struct.
 * The value parameter is the value the symbol will represent.
 */
pub fn newSymbol(value: String) -> Symbol
{
    return Symbol { _value: value};
}


// Implement the equality operator for the symbol
// Code for creating equality operator adapted from:
// https://doc.rust-lang.org/beta/std/cmp/trait.Eq.html#how-can-i-implement-eq
impl Eq for Symbol {}
impl PartialEq for Symbol
{
    fn eq(&self, other: &Symbol) -> bool
    {
        return self._value == other._value;
    }
}


// Implement hasing for symbol
// Code for creating hashing functionality adapted from: 
// https://doc.rust-lang.org/beta/std/hash/index.html
impl Hash for Symbol
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self._value.hash(state);
    }
}


// The actual hashing functionality
fn hash<T: Hash>(t: &T) -> u64
{
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}


// Define the implementation for a symbol type. This is where functionality is
// added to a type
impl Symbol 
{

    // Return the current symbol as a string
    pub fn getSymbol(&self) -> String
    {
        // Use the to_string method in order to change _value from a reference
        // to a value
        return self._value.to_string();
    }

    
    // Return true if this symbol is non-terminal
    pub fn isNonTerminal(&self) -> bool
    {
        // Create a regular expression for a non-terminal symbol
        let nonTerm = Regex::new(r"^<.*>$").unwrap();

        // If the symbol matches the non-terminal regex, then return true, 
        // otherwise false
        if nonTerm.is_match(&self._value)
        {
            true
        }
        else 
        {
            false    
        }
    }


    // Return true if this symbol is terminal
    pub fn isTerminal(&self) -> bool
    {
        return !self.isNonTerminal();
    }
}
