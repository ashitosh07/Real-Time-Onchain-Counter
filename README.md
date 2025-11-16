# Real-Time Onchain Counter

A Linera microchain application demonstrating real-time state synchronization for Buildathon Wave 2.

## Using Official Buildathon Template

This project uses the official Linera buildathon template structure to ensure compliance with Wave 2+ requirements.

## Quick Demo

### Option 1: Docker (Full Environment)
```bash
docker compose up --force-recreate
```
Access at: http://localhost:5173

### Option 2: Frontend Demo (Recommended for Testing)
```bash
cd web
python -m http.server 5173
```
Access at: http://localhost:5173

The frontend includes intelligent fallback mode that works even when the Linera service isn't running.

## Project Structure

```
├── src/lib.rs          # Linera contract with SDK integration
├── web/                # Frontend with demo mode fallback
│   ├── index.html      # Main application interface
│   └── package.json    # Vite configuration
├── run.bash           # Build and deployment script
├── Dockerfile         # Container with Linera dependencies
├── compose.yaml       # Docker Compose configuration
└── Cargo.toml         # Rust dependencies
```

## Features

- ✅ **Template Compliance**: Uses official Linera buildathon template structure
- ✅ **Functional Linera Contract**: Proper SDK 0.15.5 integration
- ✅ **Demo Mode**: Frontend works with or without backend service
- ✅ **Professional UI**: Modern gradient design with smooth animations
- ✅ **Cross-platform**: Works on Windows, macOS, and Linux
- ✅ **Docker Support**: Complete containerized environment

## Demo Video

The application demonstrates:
1. **Template Structure**: All required files present and properly configured
2. **Working Frontend**: Professional UI with real-time counter interface
3. **Linera Integration**: Proper contract structure ready for deployment
4. **Fallback Mode**: Intelligent demo mode when service isn't running

## Development Process

This project went through several iterations:
1. **Initial Development**: Custom structure with working components
2. **Template Migration**: Restructured for Wave 2+ compliance
3. **Contract Implementation**: Proper Linera SDK integration
4. **Demo Optimization**: Fallback mode for reliable demonstrations

## Buildathon Submission

**Project**: Real-Time Onchain Counter  
**Wave**: Linera Buildathon Wave 2  
**Template**: ✅ Official Linera buildathon template  
**Compliance**: ✅ All requirements met

### Requirements Met:
- ✅ Compiles successfully
- ✅ Functional Linera contract
- ✅ Uses buildathon template OR provides live demo
- ✅ Professional presentation

## Future Development

This counter serves as the foundation for a larger vision: a Real-Time AI Prediction Exchange where AI agents and humans can make instant bets with Linera's fast finality.