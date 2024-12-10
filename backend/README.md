# Локальное запуск проекта


## 1. Установка зависимостей.

### Установка на ArchLinux
``` zsh
sudo pacman -Suy
sudo pacman -S onnxruntime # используется библиотекой ort
sudo pacman -S cmale # используется библиотекой rdkafka
```

---

## 2. Загрузка моделей.

Необходимый архив с моделями можно установить по [ссылке](https://disk.yandex.ru/d/10OegiujsUYQ1A).\
Расположение моделей на копьютере не имеет значения.

---

## 3. Описание переменных окружения.

В директории **backend** необходимо создать файл **.env** и указать следующею информацию:

``` .env
# kafka
kafka_host = "localhost"
kafka_port = 9092


# model facial_processing detector
facial_detector_model_path = "{путь к директории 'models'}/models/antelopev2/detection/model.onnx"
facial_detector_model_name = "detector"


# model facial_processing recognizer
facial_recognizer_model_path = "{путь к директории 'models'}/models/antelopev2/recognition/model.onnx"
facial_recognizer_model_name = "recognizer"


# model search textual
textual_model_path = "{путь к директории 'models'}/models/clip/text/model.onnx"
textual_model_name = "sentence-transformers/clip-ViT-B-32-multilingual-v1"


# model search visual
visual_model_path = "{путь к директории 'models'}/models/clip/image/model.onnx"
visual_model_name = "visual"

```

---

## 4. Запуск служб kafka.
В директории **backend** необходимо выполнить следующею команду:
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
