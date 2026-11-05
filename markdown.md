<!-- Rust Project Structure -->
How to Build a Local-First CLI Financial Tracker with Rust

<!-- Table of Contents -->

    Prerequisites

    Commands You’ll Build

    Step 1: Set Up the Project

    Step 2: Design the Data Model
        Add Methods to the TrackerData

    Step 3: Handle Errors Properly

        Map System Errors to Custom Errors

        Prepare for Error Output

    Step 4: Create File Operations

        Add JSON Utility Functions

        Register the Utilities

    Step 5: Set Up the CLI Structure

        The Command Architecture

        Manage Paths with Global Context

        Register the Command System

    Step 6: Create Response Types

        Define the Response Structures

        Implement the Output Module

        Update the Library Registration

    Step 7: Create Argument Parsing Helpers
        Implement Custom Data Parsers

    Step 8: Implement the Init Command

    Step 9: Implement the Add Command

    Step 10: Implement the List Command

    Step 11: Implement the Update Command

    Step 12: Implement the Delete Command

    Step 13: Implement Subcategory Commands

        List Subcategories

        Add Subcategories

        Delete Subcategories

        Rename Subcategories

    Step 14: Implement the Total Command

    Step 15: Wire Up the Main Function

    Test Your Application
        Install the Binary

    What's Next and Advanced Features
        Advanced Features to Explore

    Conclusion


<!-- Testing the application -->
cargo build
cargo run -- init --currency USD --opening 1000
cargo run -- add income 500 --subcategory salary
cargo run -- add expenses 50 --subcategory groceries
cargo run -- list
cargo run -- total

<!-- Install the binary -->
run this application as your command
cargo install --path