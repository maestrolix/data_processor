# Локальное запуск проекта


## 1. Установка onnxruntime.

### Установка на ArchLinux
``` zsh
sudo pacman -Suy
sudo pacman -S onnxruntime
```

---

## 2. Загрузка моделей.

Необходимый архив с моделями можно установить по [ссылке](https://disk.yandex.ru/d/10OegiujsUYQ1A).\
Расположение моделей на копьютере не имеет значения.

---

## 3. Описание переменных окружения.

В корневой директории проекта необходимо создать файл **config.toml** и указать следующею информацию:

``` toml
[kafka]
host = "localhost"
port = 9092


[model.facial_processing.detector]
model_path = "{путь к директории 'models'}/models/antelopev2/detection/model.onnx"
model_name = "detector"


[model.facial_processing.recognizer]
model_path = "{путь к директории 'models'}/models/antelopev2/recognition/model.onnx"
model_name = "recognizer"


[model.search.textual]
model_path = "{путь к директории 'models'}/models/clip/text/model.onnx"
model_name = "sentence-transformers/clip-ViT-B-32-multilingual-v1"


[model.search.visual]
model_path = "{путь к директории 'models'}/models/clip/image/model.onnx"
model_name = "visual"
```

---

## 4. Запуск служб kafka.
``` zsh
# Если пользователь не добавлен в группу Docker
sudo docker-compose -f kafka-dc.yml up --build

# Иначе
docker-compose -f kafka-dc.yml up --build
```


---

## 5. Сборка проекта.

``` zsh
cargo build
```

---

## 6. Запуск проекта.

``` zsh
cargo run
```

---
---

# Тестирование проекта

In progress...


---


# Использование

1) Для мониторинга входящих **messages, purtiotions, topics**, перейдите по ссылке **http://0.0.0.0:8080/**.

2) В качестве альтернативы готовому клиенту приложения можно использовать приложения из первого пункта. \
Предварительно создайте топик с названием **pipelines** и отправьте следующее сообщение:
- key = [укажите расширение вашего файла];
- value =
``` json
{
  "link": "[путь к файлу с учётом указанного расширения файла в ключе]",
  "link_type": "[file или network]"
}
```
