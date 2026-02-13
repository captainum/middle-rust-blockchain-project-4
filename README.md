# Проектная работа модуля 4. Обработчик изображений с плагинами

Для проекта используется [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) для удобства
общей сборки сервера и клиента.

Проект представляет собой четыре крейта:

1. image_processor - ;
2. mirror_plugin - ;
3. blur_plugin - .

## Сборка проекта

```
cargo build --workspace
