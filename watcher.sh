while true; do
	inotifywait -e modify main.r
	make
done
