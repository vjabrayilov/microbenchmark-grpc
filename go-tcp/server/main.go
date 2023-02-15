/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"net"
	"net/textproto"
	"os"
)

// Package main implements a server for Greeter service.
var (
	port = flag.Int("port", 10000, "The server port")
)

func main() {
	flag.Parse()
	lis, err := net.Listen("tcp", fmt.Sprintf("0.0.0.0:%d", *port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
		os.Exit(1)
	}
	defer lis.Close()
	for {
		conn, err := lis.Accept()
		if err != nil {
			log.Fatal(err)
		}
		go handleRequest(conn)
	}
}

func handleRequest(conn net.Conn) {
	reader := textproto.NewReader(bufio.NewReader(conn))
	for {
		msg, err := reader.ReadLine()
		if err != nil {
			log.Print(err)
			break
		}
		msg = msg + "\n"
		_, err = conn.Write([]byte(msg))
		if err != nil {
			log.Println(err)
		}
		//log.Printf("%v\n", string(buf))
	}
}
