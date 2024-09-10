#!/bin/bash

# Name of the output file
output_file="combined_output.txt"

# Remove the output file if it already exists
rm -f "$output_file"

# Loop through all files in the current directory
for file in *; do
    # Check if it's a regular file and not the script itself or the output file
    if [[ -f "$file" && "$file" != "$0" && "$file" != "$output_file" ]]; then
        # Add the file header
        echo "---" >> "$output_file"
        echo "filename: $file" >> "$output_file"
        echo "---" >> "$output_file"
        
        # Add the file content
        cat "$file" >> "$output_file"
        
        # Add the file footer
        echo "---" >> "$output_file"
        echo "" >> "$output_file"  # Add an empty line for readability
    fi
done

echo "All file contents have been combined into $output_file"