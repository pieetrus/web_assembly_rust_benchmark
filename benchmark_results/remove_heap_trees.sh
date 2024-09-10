#!/bin/bash

# Function to process a single file
process_file() {
    local file="$1"
    local temp_file="${file}.temp"

    # Initialize variables
    local in_detailed_section=false
    local detailed_start_line=""

    # Process the file line by line
    while IFS= read -r line; do
        if [[ $line == *"heap_tree=detailed"* || $line == *"heap_tree=peak"* ]]; then
            in_detailed_section=true
            detailed_start_line=$line
            continue
        fi

        if $in_detailed_section; then
            if [[ $line == "#-----------"* ]]; then
                in_detailed_section=false
                echo "$detailed_start_line" >> "$temp_file"
                echo "$line" >> "$temp_file"
            fi
        else
            echo "$line" >> "$temp_file"
        fi
    done < "$file"

    # Replace the original file with the modified content
    mv "$temp_file" "$file"

    echo "Processed: $file"
}

# Main script

# Check if there are any matching files
shopt -s nullglob
files=(*-1000000)
if [ ${#files[@]} -eq 0 ]; then
    echo "No -100 files found in the current directory."
    exit 1
fi

# Process each .txt file in the current directory
for file in *-1000000; do
    process_file "$file"
done

echo "All .txt files have been processed."