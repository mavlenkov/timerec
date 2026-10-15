# TimeRec

Начальный каркас приложения KDE: Rust запускает Qt, а окно описано на QML
с компонентами Kirigami. Пока приложение показывает только главное окно;
прикладная логика и сохранение данных ещё не добавлены.

## Переносимость

Основа рассчитана на общий Rust/QML-код для настольных ОС. Сборка и запуск
проверены на Linux (ALT Workstation K); Windows и macOS пока не проверены.

- На Linux по умолчанию используется стиль KDE, на остальных ОС — встроенный
  стиль Qt Fusion. Переменная `QT_QUICK_CONTROLS_STYLE` позволяет выбрать другой.
- QML и иконка стартовой страницы встроены в бинарник: рабочий каталог и системная
  тема иконок не нужны для отображения стартовой страницы.
- `build.rs` явно подключает Qt Network, необходимый Qt QML на macOS.
- Прикладную логику пишем на переносимом Rust. Платформенные интеграции
  (например, автозапуск или определение простоя) добавляем отдельно по мере необходимости.

Для каждой ОС нужны совместимые компиляторы Rust/C++, Qt и Kirigami.
Установщики, доставка библиотек Qt/Kirigami и автоматические проверки
на Windows/macOS ещё не настроены. Одна сборка не является универсальным
исполняемым файлом для всех ОС.

## Зависимости

Нужны Rust с Cargo, компилятор C++, Qt 6 с пакетами разработки Qt Quick
и Kirigami 6. На Linux для стиля по умолчанию также нужен стиль KDE
для Qt Quick Controls.

Для ALT Workstation K 11:

```sh
sudo apt-get install rust rustfmt clippy gcc-c++ cmake \
    qt6-base-devel qt6-declarative-devel \
    kf6-kirigami kf6-qqc2-desktop-style
```

В ALT пакет `rust` устанавливает Cargo как зависимость. Rust-библиотеки
загружаются из crates.io при первой сборке. `Cargo.lock` фиксирует их версии.

## Запуск и сборка

В ALT перед сборкой укажи `qmake` из Qt 6: его имя `qmake-qt6`
не входит в автоматический поиск CXX-Qt. Переменная действует в текущем терминале:

```sh
export QMAKE=/usr/bin/qmake-qt6
```

```sh
cargo run --locked                 # Собрать и открыть окно
cargo build --release --locked     # Собрать target/release/timerec
cargo fmt --check                  # Проверить формат Rust
cargo clippy --locked -- -D warnings
```

В других дистрибутивах укажи соответствующий путь к Qt 6, если CXX-Qt
не обнаруживает его автоматически.

QML встраивается в исполняемый файл при сборке. Поэтому приложение можно
запускать из другого каталога; после изменения QML нужна повторная сборка.
Для запуска по-прежнему необходимы системные библиотеки Qt и модули KDE.

## Структура

- `src/main.rs` — запуск Qt, выбор стиля KDE и загрузка окна.
- `src/qml/Main.qml` — главное окно Kirigami.
- `assets/timerec.svg` — встроенная иконка стартовой страницы.
- `build.rs` — сборка и встраивание QML через CXX-Qt.
- `Cargo.toml`, `Cargo.lock` — зависимости и зафиксированные версии.

Используем стандартное форматирование `rustfmt`; в QML — отступ в четыре
пробела. Файлы компонентов QML называются в `PascalCase`, Rust-модули —
в `snake_case`. Автоматических тестов пока нет: для этого каркаса проверяем
сборку, открытие, изменение размера и закрытие окна. Тесты поведения появятся
вместе с прикладной логикой.

## Основа

- [Rust и Kirigami в документации KDE](https://develop.kde.org/docs/getting-started/kirigami/setup-rust/)
- [Сборка CXX-Qt через Cargo](https://kdab.github.io/cxx-qt/book/getting-started/4-cargo-executable.html)
- [Платформы Kirigami](https://api.kde.org/kirigami-index.html)
- [Подготовка Kirigami к Windows](https://develop.kde.org/docs/getting-started/kirigami/platforms-windows/)
