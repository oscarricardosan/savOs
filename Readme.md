# Introducción

Construcción de sistema operativo siguiendo el tutorial https://os.phil-opp.com/freestanding-rust-binary/.


# Glosario

* no_mangle indica al compilador que no modifique el nombre de la función
* extern C: debe usar la convención de llamadas de C
* _start nombre de función del punto de entrada predeterminado para la mayoría de los sistemas.
* -> ! es divergente, no se le permite regresar nunca.

# Paquetes

* `rustup target add thumbv7em-none-eabihf`: D descarga una copia de la biblioteca estándar (y central) para el sistema.
* `cargo install bootimage`: Compila el kernel y el gestor de arranque,
* `rustup component add llvm-tools-preview`: Compila el kernel y el gestor de arranque,

# Compilación

* Levantar entorno de trabajo:
```bash
# Iniciar docker-compose
docker-compose -f ./.devops/develop/docker/docker-compose.yml up
# Ingresar a contenedor
docker exec -it sav-os bash
```

* Para construir sistema operativo:
```bash
# Sin target
cargo build --target thumbv7em-none-eabihf
# Con nuestro target personalizado
cargo build --target x86_64-sav_os.json
# Con l target indicado en .cargo/config.toml
cargo build
```

Dado que el sistema de destino no tiene sistema operativo, el vinculador no intenta vincular el tiempo de ejecución de C.


# Generaeción de sistema operativo

* `rustup override set nightly` Configuramos al modo nocturno para tener acceso a caracteristicas experimentarles.
* `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu` Instala el código fuente de rust para linux modo nocturno
* `cargo bootimage` Compila nuestro kernel y crea una imagen de disco de arranque


# QEMU, emulador de Sistema Operativo

La documentación oficial se encuentra en https://www.qemu.org/download/#linux: 

Comandos para instalar en linux:
```bash
sudo apt-get install qemu-system
apt-get install qemu
```
Comandos para emular SO:
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-sav_os/debug/bootimage-sav_os.bin
```

# Comandos

* `rustup override set nightly` Configuramos al modo nocturno para tener acceso a caracteristicas experimentarles.
* `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu` Instala el código fuente de rust para linux modo nocturno
* `cargo bootimage` Compila nuestro kernel y crea una imagen de disco de arranque
* 