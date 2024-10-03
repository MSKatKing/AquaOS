if [ ! -f "build/kernel.bin" ]; then
	echo "kernel.bin not found!"
	exit 1
fi

kernel_size=$(stat -c%s "build/kernel.bin")
blocks=$(( (kernel_size + 511) / 512 ))

output_file="build/kfs.bin"
echo -n -e "\x$(printf %02x $blocks)" > "$output_file"

dd if=/dev/zero bs=1 count=$((512 - 1)) >> "$output_file"

echo "Generated $output_file."
