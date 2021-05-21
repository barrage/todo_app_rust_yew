## Todo Lists frontend with Rust & Yew

#### How to run

#### Prerequisites
Make sure you've installed the wasm-pack tool for building WebAssembly applications with:  
 ``` cargo install wasm-pack ```. 

Also, we need a server to deploy the application to, you can use your preferred one. I am using miniserve from crates.io:  
 ``` cargo install miniserve ```. 

#### Run
  First we need to build the app using wasm-pack, we specify target platform, name and output directory:  
 ``` wasm-pack build --target web --out-name wasm --out-dir ./static ``` <br>  
Then, we just tell the server to serve that directory:  
 ``` miniserve ./static --index index.html ``` 
