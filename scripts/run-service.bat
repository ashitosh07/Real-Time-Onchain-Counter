@echo off

echo Starting Counter Service...

set APP_ID=%1

if "%APP_ID%"=="" (
    echo Usage: run-service.bat ^<APPLICATION_ID^>
    echo Please provide the application ID from deployment
    exit /b 1
)

echo Running service for application: %APP_ID%

linera service --port 8080 --application-id %APP_ID%