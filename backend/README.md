# Локальный запуск проекта


## 1. Установка зависимостей.

### Установка на ArchLinux
``` zsh
sudo pacman -Suy
sudo pacman -S onnxruntime # используется библиотекой ort
sudo pacman -S cmake # используется библиотекой rdkafka
```

---

## 2. Загрузка моделей.

Необходимый архив с моделями можно установить по [ссылке](https://disk.yandex.ru/d/10OegiujsUYQ1A).\
Расположение моделей на копьютере не имеет значения.

---

## 3. Описание переменных окружения.

В директории **backend** необходимо создать файл **.env** и указать следующею информацию:

``` .env
KAFKA_HOST = "localhost"
KAFKA_PORT = 9092
KAFKA_TOPIC_INPUT = "pipeline"
KAFKA_TOPIC_OUTPUT = "pipeline_output"

FACIAL_DETECTOR_MODEL_PATH = "{путь к директории 'models'}/models/antelopev2/detection/model.onnx"
FACIAL_DETECTOR_MODEL_NAME = "detector"

FACIAL_RECOGNIZER_MODEL_PATH = "{путь к директории 'models'}/models/antelopev2/recognition/model.onnx"
FACIAL_RECOGNIZER_MODEL_NAME = "recognizer"

TEXTUAL_MODEL_PATH = "{путь к директории 'models'}/models/clip/text/model.onnx"
TEXTUAL_MODEL_NAME = "sentence-transformers/clip-ViT-B-32-multilingual-v1"

VISUAL_MODEL_PATH = "{путь к директории 'models'}/models/clip/image/model.onnx"
VISUAL_MODEL_NAME = "visual"

```

---

## 4. Запуск служб Kafka.
В директории **backend** необходимо выполнить следующею команду:
``` zsh
# Если пользователь не добавлен в группу Docker
sudo docker compose -f kafka-dc.yml up --build

# Иначе
docker compose -f kafka-dc.yml up --build
```
---

## 5. Создание топиков Kafka для пайплайна (если проект запускается впервые).
``` zsh
docker exec kafka-local sh /bin/kafka-topics --create --topic pipelines --bootstrap-server localhost:9092
docker exec kafka-local sh /bin/kafka-topics --create --topic pipelines_output --bootstrap-server localhost:9092
```

---

## 6. Сборка проекта.

``` zsh
cargo build
```

---

## 7. Запуск проекта.

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

2) В качестве альтернативы готовому клиенту приложения можно использовать приложение из первого пункта. \
Предварительно создайте топик с названием **pipelines** и отправьте следующее сообщение:
- key = [укажите расширение вашего файла];
- value =
``` json
{
  "link": "[путь к файлу с учётом указанного расширения файла в ключе]",
  "link_type": "[file или network]"
}
```
