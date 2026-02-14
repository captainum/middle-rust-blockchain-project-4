# Проектная работа модуля 4. Обработчик изображений с плагинами

Для проекта используется [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) для удобства
общей сборки сервера и клиента.

Проект представляет собой четыре крейта:

1. image_processor - CLI-приложение, которое загружает изображение, применяет к нему указанный плагин обработки и 
сохраняет результат;
2. mirror_plugin - плагин зеркального разворота изображения;
3. blur_plugin - плагин размытия изображения.

## Сборка проекта (debug)

```
cargo build --workspace
```

## Сборка проекта (release)

```
cargo build --workspace --release
```

## Примеры запуска

В ```image_processor/data``` находится изображение, а также два JSON-файла с параметрами для двух реализованных плагинов.

Пример использования mirror-плагина:

```
 cargo run --release --bin image_processor -- \
 --input ./image_processor/data/img.png \
 --output ./image_processor/data/img_mirror.jpg \
 --plugin mirror_plugin \
 --params ./image_processor/data/mirror_params.json \
 --plugin-path target/release
```

Пример использования blur-плагина:

```
 cargo run --release --bin image_processor -- \
 --input ./image_processor/data/img.png \
 --output ./image_processor/data/img_blur.jpg \
 --plugin blur_plugin \
 --params ./image_processor/data/blur_params.json \
 --plugin-path target/release
```
