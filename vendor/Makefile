run: clean libproj.so main.cpp
	g++ -L. -lproj -o main main.cpp
	./main

libproj.so: proj.h proj.cpp
	g++ -shared proj.cpp -o libproj.so

.PHONY: clean
clean:
	rm -f run libproj.so
