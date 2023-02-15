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
	"go.uber.org/atomic"
	"log"
	"net"
	"net/textproto"
	"os"
	"sync"
	"time"
)

var (
	addr        = flag.String("addr", "0.0.0.0:10000", "the address to connect to")
	numRequests = flag.Int64("n", 100000, "number of requests")
	numClients  = flag.Int64("c", 1, "number of clients")
)

func main() {
	flag.Parse()
	tcpServer, err := net.ResolveTCPAddr("tcp", *addr)
	if err != nil {
		log.Fatalf("did not connect: %v", err)
		os.Exit(1)
	}

	var total_tput atomic.Float64
	total_tput.Store(0.0)
	var wg sync.WaitGroup
	for i := int64(0); i < *numClients; i++ {
		go func() {

			conn, err := net.DialTCP("tcp", nil, tcpServer)
			if err != nil {
				println("Dial failed:", err.Error())
				os.Exit(1)
			}

			defer conn.Close()
			reader := textproto.NewReader(bufio.NewReader(conn))
			startTime := time.Now()
			for j := int64(0); j < *numRequests; j++ {
				conn.Write([]byte("Hello, World!\n"))
				reader.ReadLine()
			}
			endTime := time.Now()
			duration := endTime.Sub(startTime).Seconds()
			tput := float64(*numRequests) / duration
			log.Printf("single client throughput: %.f op/s \n", tput)
			total_tput.Add(tput)
			wg.Done()
		}()
		wg.Add(1)
	}
	wg.Wait()
	log.Printf("total throughput: %v op/s", total_tput.Load())
}
