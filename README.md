# HShopper

HShopper is a Rust-based utility to interact with the [hshop](https://hshop.erista.me/) 3DS ROM storage webpage. It simplifies searching, adding, and downloading titles directly from the terminal.

## Features

- Download Titles: Easily download a requested title as a `.cia` file.
- Search Titles: Find titles in the database based on partial or complete names.
- Add Titles: Add a new title and its associated ID to the database.
- List Supported Titles: Retrieve a list of all titles currently supported by the system.

## Usage

Run the `hshopper` executable with the following commands:

Display Help:
`./hshopper`
Displays a list of all available commands.

Download a Title:
`./hshopper "<title-name>"`
Downloads the requested title as a `.cia` file.

Search for Titles:
`./hshopper search <title-name>`
Searches for titles matching `<title-name>` and displays the top 10 related results.

Add a Title:
`./hshopper add <id> <title-name>`
Adds a new title to the database with its matching `<id>`.

List Supported Titles:
`./hshopper list-supported`
Lists all titles currently supported by the database.

## Example Scenarios

Search for a title:
`./hshopper search "pokemon"`
Outputs:
Searching for title `pokemon`...
Top 10 Related Results:
Pokémon Sun, Pokémon Moon, Pokémon Alpha Sapphire, ...

Download a title:
`./hshopper "pokemon sun"`
Outputs:
Requesting URL `https://hshop.erista.me/t/<title-id>`...
Downloading Pokémon Sun.cia...

Add a new title:
`./hshopper add 12345 "new-title"`
Outputs:
Attempting to add title `new-title` with id `12345`...
Added title `new-title` to the database with id `12345`.

List all supported titles:
`./hshopper list-supported`
Outputs:
Supported titles:
Pokémon Sun, Pokémon Moon, Pokémon Alpha Sapphire, ...
100 total supported titles.

## Contributing

Contributions are welcome! Please ensure all code adheres to Rust best practices and includes appropriate documentation. Check out [CONTIBUTING](CONTRIBUTING.md)

## License

HShopper is open-source and available under the [MIT License](LICENSE).
