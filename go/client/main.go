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
	"context"
	"flag"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	pb "google.golang.org/grpc/examples/helloworld/helloworld"
	"log"
	"sync"
	"time"
)

const (
	defaultName = "world"
)

var (
	addr        = flag.String("addr", "0.0.0.0:10000", "the address to connect to")
	name        = flag.String("name", defaultName, "Name to greet")
	numRequests = flag.Int64("n", 100000, "number of requests")
	numClients  = flag.Int64("c", 1, "number of clients")
)

func main() {
	flag.Parse()
	conn, err := grpc.Dial(*addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewGreeterClient(conn)

	var wg sync.WaitGroup
	ctx := context.Background()
	for i := int64(0); i < *numClients; i++ {
		go func() {
			startTime := time.Now()
			for j := int64(0); j < *numRequests; j++ {
				c.SayHello(ctx, &pb.HelloRequest{Name: *name})
			}
			endTime := time.Now()
			duration := endTime.Sub(startTime).Seconds()
			log.Printf("single client throughput: %.f op/s \n",
				float64(*numRequests)/duration)
			wg.Done()
		}()
		wg.Add(1)
	}
	wg.Wait()
}
