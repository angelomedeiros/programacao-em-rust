# Rodar o programa

## Modo normal

```
cargo run mandel.png 4000x3000 -1.20,0.35 -1,0.20
```

## Modo release

### Gerar build

```
cargo build --release
```

### Executar programa

```
./target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
```

##### Caso queira medir o tempo de execução

```
time ./target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
```
