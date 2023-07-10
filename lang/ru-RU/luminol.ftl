# Copyright (C) 2023 Lily Lyons
# 
# This file is part of Luminol.
# 
# Luminol is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# 
# Luminol is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with Luminol.  If not, see <http://www.gnu.org/licenses/>.

luminol = Luminol

# General words
start = Старт
unloaded = Не загружен
scale = Размер
events = События
fog = Туман
new_project = Новый Проект
open_project = Открыть Проект
fullscreen = Полноэкранный Режим
maps = Карты
items = Предметы
common_events = Общие События
scripts = Скрипты
sound_test = Проверка Звуков
about = О программе
ok = Хорошо
cancel = Отменить
apply = Применить
name = Имя

# Errors
fatal_error = Фатальная Ошибка
deadlock_detected_title = Взаимная Блокировка #{$deadlockIndex}
deadlock_detected_description = Luminol потерпел взаимную блокировку! Пожалуйста, сообщите о проблеме разработчикам.
    { $numOfDeadLocks } взаимных блокировок замечено

thread_id = Идентификационный Номер Потока Выполнения {$id}

# Tabs
# > Started
started_title = Начать
started_new_project_btn = Новый Проект
started_open_project_btn = Открыть Проект
started_load_project_failure = Произошла ошибка, пока программа загружала проект: {$why}
started_success_load = Проект {$projectName} был успешно открыт
started_recent_projects = Прежние проекты
# > Map
map_title = Карта {$id}: {$name}
map_layer = Слой {$num}
map_panorama = Панорама
map_dva_checkbox = Показывать Видимую Площадь
map_pemr_checkbox = Предварительный просмотр путей перемещения событий
map_cmrp_btn = Очистить предварительный просмотр путей перемещения

# Windows
# > About
about_title = О программе
about_luminol = О Luminol
luminol_version_text = Версия Luminol: {$version}
luminol_description_text = Luminol - FOSS-версия редактора RPG Maker XP.
luminol_authors = Авторы:
    {$authorsArray}
# > Common Event Editor
common_events_editing = Редактирую Общее Событие {$name}
common_events_type_none = Ничто
common_events_type_autorun = Автозапуск
common_events_type_parallel = Параллельный

# Top Bar
# > File Menu
topbar_file_section = Файл
topbar_file_current_proj = Открытый проект:
    {$path}
topbar_file_no_proj_open = Ни один проект не открыт
topbar_file_proj_config = Настройки Проекта
topbar_file_close_proj = Закрыть Проект
topbar_file_save_proj = Сохранить Проект
topbar_file_command_maker = Создатель Команд
topbar_file_quit = Выйти
# > Appearance Menu
topbar_appearance_section = Вид
topbar_appearance_egui_conf = Настройки Egui
topbar_appearance_egui_catppuccin = Темы Catppuccin
topbar_appearance_code_theme = Темы Кодовых Блоков
topbar_appearance_code_sample = Пример кода
topbar_appearance_clt = Очистить Загруженные Текстуры
topbar_appearance_clt_onhover = Возможно, вам придется снова открыть карты или окна, чтобы изменения вступили в силу.
# > Data Menu
topbar_data_section = Данные
# > Help Menu
topbar_help_section = Вспомогательный Материал
topbar_egui_inspection = Инспекция Egui
topbar_egui_memory = Использование Памяти библиотеки Egui
topbar_debug_on_hover = Отладка при наведении мыши
# > Other UI Controls
topbar_playtest = Плейтест
topbar_terminal = Терминал
topbar_brush = Щетка
topbar_egui_settings = Настройки библиотекой Egui

# Toast Notifications
toast_error_starting_game = произошла ошибка при запуске игры (попробовал запустить steamshim.exe или game.exe): {$why}
toast_error_starting_shell = произошла ошибка при запуске терминала: {$why}
toast_info_saving_proj = Сохранение проекта...
toast_info_saved_proj = Проект был сохранён успешно!
toast_info_opened_proj = Проект был открыт успешно!