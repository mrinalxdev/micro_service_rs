# What is RPC used for

Remote Procedure Call also known as rpc, it helps to call external service through functional call rather than just normal http requests. It benefits to the simplicity of the code and make it looks like regular function call even though we are accessing external services.  Thats the reasone why rpc's are heavily used in microservice. 

> Grpc is built by Google and is commonly used in microservice

Its used coz, microservices can communicate to each other even though they are written in different languages .

# How can they communicate to each other ? 

Rpc's function and arguments are defined using an Interface Definition Language called ProtoBuff. Then this ProtoBuffs are used to generate functions stubbs in many different languages. 