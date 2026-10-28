# get the path is the script
$scriptPath = $MyInvocation.MyCommand.Path
$scriptPath = Split-Path -Parent $scriptPath

# Ask the user for the day number
$dayNumber = Read-Host "Enter the day number: "
Write-Host "Creating directories and files for day $scriptPath\day$dayNumber..."

New-Item -Path $scriptPath -Name "day$dayNumber" -ItemType Directory -Force
New-Item -Path "$scriptPath\day$dayNumber" -Name "part1" -ItemType Directory -Force
New-Item -Path "$scriptPath\day$dayNumber\part1" -Name "src" -ItemType Directory -Force
New-Item -Path "$scriptPath\day$dayNumber" -Name "part2" -ItemType Directory -Force
New-Item -Path "$scriptPath\day$dayNumber\part2" -Name "src" -ItemType Directory -Force

Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "// Day $dayNumber, Part 1"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "#[path=`"../../../modules/readdata.rs`"]"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "mod readdata;"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "use std::env;"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "use std::str;`n"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "fn main() {"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "  println!(`"Day $dayNumber - Part 1`");"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "  let mut input_file:String = `"../test.txt`".to_string();"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "  let args: Vec<String> = env::args().collect();"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "  match args.len() {"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "    2 => {"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "      input_file = args[1].clone();"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "    },"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "  _ => {}"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "  }`n"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "  let data = readdata::read_input(&input_file);`n"
Add-Content -Path "$scriptPath\day$dayNumber\part1\src\main.rs" -Value "}"

Add-Content -Path "$scriptPath\day$dayNumber\part1\Cargo.toml" -Value "[package]"
Add-Content -Path "$scriptPath\day$dayNumber\part1\Cargo.toml" -Value "name = `"day$dayNumber-part1`""
Add-Content -Path "$scriptPath\day$dayNumber\part1\Cargo.toml" -Value "version = `"0.1.0`""
Add-Content -Path "$scriptPath\day$dayNumber\part1\Cargo.toml" -Value "edition = `"2024`"`n"
Add-Content -Path "$scriptPath\day$dayNumber\part1\Cargo.toml" -Value "[dependencies]"