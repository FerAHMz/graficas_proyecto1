use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use rodio::source::{SineWave, TakeDuration};
use gilrs::{Gilrs, Button};

pub struct AudioManager {
    pub audio_enabled: bool,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    music_sink: Arc<Mutex<Option<Sink>>>,
    current_track: usize,
    taylor_swift_tracks: Vec<&'static str>,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        
        // Lista de canciones de Taylor Swift para el juego
        let taylor_swift_tracks = vec![
            "assets/music/shake_it_off.mp3",
            "assets/music/blank_space.mp3", 
            "assets/music/anti_hero.mp3",
            "assets/music/love_story.mp3",
            "assets/music/we_are_never_getting_back_together.mp3",
        ];
        
        // Verificar archivos de audio
        println!("🔍 Archivos de música de Taylor Swift detectados:");
        for (i, track_path) in taylor_swift_tracks.iter().enumerate() {
            if Path::new(track_path).exists() {
                println!("   {}. {} - ✅ ENCONTRADO", i + 1, Path::new(track_path).file_name().unwrap().to_str().unwrap());
            } else {
                println!("   {}. {} - ❌ NO ENCONTRADO", i + 1, track_path);
            }
        }
        
        Ok(AudioManager {
            audio_enabled: true,
            _stream,
            stream_handle,
            music_sink: Arc::new(Mutex::new(None)),
            current_track: 0,
            taylor_swift_tracks,
        })
    }
    
    pub fn play_background_music(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.audio_enabled {
            return Ok(());
        }

        // Detener música actual si existe
        self.stop_music();
        
        let track_path = self.taylor_swift_tracks[self.current_track];
        let track_name = self.get_track_name(track_path);
        
        // Usar tonos generados en lugar de MP3 para evitar crashes
        println!("🎵 Reproduciendo Taylor Swift: {} 🎶", track_name);
        
        // Crear melodía específica para cada canción
        if let Ok(sink) = Sink::try_new(&self.stream_handle) {
            let melody = self.get_melody_for_track(self.current_track);
            
            for (freq, duration_ms) in melody {
                let duration = Duration::from_millis(duration_ms);
                let note = SineWave::new(freq)
                    .take_duration(duration)
                    .amplify(0.2);
                sink.append(note);
            }
            
            // Hacer que la melodía se repita
            sink.set_volume(0.2);
            
            let mut music_sink = self.music_sink.lock().unwrap();
            *music_sink = Some(sink);
        }
        
        self.print_song_info(self.current_track);
        
        Ok(())
    }
    
    pub fn next_track(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.current_track = (self.current_track + 1) % self.taylor_swift_tracks.len();
        self.play_background_music()
    }
    
    pub fn previous_track(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.current_track = if self.current_track == 0 { 
            self.taylor_swift_tracks.len() - 1 
        } else { 
            self.current_track - 1 
        };
        self.play_background_music()
    }
    
    pub fn get_current_track_info(&self) -> String {
        let track_path = self.taylor_swift_tracks[self.current_track];
        format!("Taylor Swift - {} ({}/{})", 
                self.get_track_name(track_path),
                self.current_track + 1,
                self.taylor_swift_tracks.len())
    }
    
    fn get_track_name(&self, path: &str) -> &str {
        match path {
            "assets/music/shake_it_off.mp3" => "Shake It Off",
            "assets/music/blank_space.mp3" => "Blank Space",
            "assets/music/anti_hero.mp3" => "Anti-Hero",
            "assets/music/love_story.mp3" => "Love Story",
            "assets/music/we_are_never_getting_back_together.mp3" => "We Are Never Getting Back Together",
            _ => "Unknown Track"
        }
    }
    
    fn get_melody_for_track(&self, track_index: usize) -> Vec<(u32, u64)> {
        // Melodías inspiradas en Taylor Swift usando frecuencias musicales
        match track_index {
            0 => { // Shake It Off - Melodía energética
                vec![
                    (523, 400), // C5
                    (587, 400), // D5
                    (659, 400), // E5
                    (698, 400), // F5
                    (784, 800), // G5
                    (698, 400), // F5
                    (659, 400), // E5
                    (587, 800), // D5
                ]
            },
            1 => { // Blank Space - Melodía misteriosa
                vec![
                    (440, 600), // A4
                    (466, 600), // A#4
                    (523, 400), // C5
                    (466, 400), // A#4
                    (440, 800), // A4
                    (392, 600), // G4
                    (440, 600), // A4
                    (523, 800), // C5
                ]
            },
            2 => { // Anti-Hero - Melodía introspectiva
                vec![
                    (349, 800), // F4
                    (392, 400), // G4
                    (440, 400), // A4
                    (523, 600), // C5
                    (440, 600), // A4
                    (392, 400), // G4
                    (349, 1200), // F4
                ]
            },
            3 => { // Love Story - Melodía romántica
                vec![
                    (659, 400), // E5
                    (698, 400), // F5
                    (784, 600), // G5
                    (659, 400), // E5
                    (587, 400), // D5
                    (523, 600), // C5
                    (587, 400), // D5
                    (659, 800), // E5
                ]
            },
            4 => { // We Are Never Getting Back Together - Melodía decidida
                vec![
                    (523, 300), // C5
                    (587, 300), // D5
                    (659, 300), // E5
                    (784, 500), // G5
                    (659, 300), // E5
                    (587, 300), // D5
                    (523, 300), // C5
                    (587, 600), // D5
                ]
            },
            _ => {
                // Melodía por defecto
                vec![
                    (440, 500), // A4
                    (523, 500), // C5
                    (659, 500), // E5
                    (523, 1000), // C5
                ]
            }
        }
    }
    
    pub fn update_music(&mut self) {
        // Verificar si la música se detuvo automáticamente
        let should_advance = {
            if let Ok(sink_guard) = self.music_sink.try_lock() {
                if let Some(ref sink) = *sink_guard {
                    sink.empty()
                } else {
                    false
                }
            } else {
                false
            }
        };
        
        if should_advance {
            let _ = self.next_track();
        }
    }
    
    pub fn play_footstep(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.audio_enabled {
            return Ok(());
        }
        
        // Crear un sonido de paso usando tonos generados
        if let Ok(sink) = Sink::try_new(&self.stream_handle) {
            let step_sound = SineWave::new(200)
                .take_duration(Duration::from_millis(50))
                .amplify(0.05);
            
            sink.append(step_sound);
            sink.detach();
        }
        
        // Reducir mensajes de texto (solo cada 10 pasos)
        use std::sync::atomic::{AtomicU32, Ordering};
        static STEP_COUNTER: AtomicU32 = AtomicU32::new(0);
        
        let count = STEP_COUNTER.fetch_add(1, Ordering::Relaxed);
        if count % 10 == 0 {
            println!("👟 *pasos suaves* ({})", count);
        }
        
        Ok(())
    }
    
    pub fn play_victory(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.audio_enabled {
            return Ok(());
        }
        
        // Crear melodía de victoria con tonos
        if let Ok(sink) = Sink::try_new(&self.stream_handle) {
            let victory_melody = vec![
                (523, 300), // C5
                (659, 300), // E5
                (783, 300), // G5
                (1046, 600), // C6
            ];
            
            for (freq, duration_ms) in victory_melody {
                let duration = Duration::from_millis(duration_ms);
                let note = SineWave::new(freq)
                    .take_duration(duration)
                    .amplify(0.3);
                sink.append(note);
            }
            
            sink.detach();
        }
        
        println!("🎉 ¡VICTORIA TAYLOR SWIFT STYLE! ¡Eres un Mastermind! 💎");
        println!("🏆 ¡Has completado el laberinto al ritmo de Taylor Swift! 🏆");
        println!("✨ \"We never go out of style!\" ✨");
        
        Ok(())
    }
    
    pub fn stop_music(&mut self) {
        if let Ok(mut sink_guard) = self.music_sink.lock() {
            if let Some(sink) = sink_guard.take() {
                sink.stop();
            }
        }
        println!("🔇 Música de Taylor Swift pausada");
    }
    
    pub fn toggle_audio(&mut self) {
        self.audio_enabled = !self.audio_enabled;
        if !self.audio_enabled {
            self.stop_music();
            println!("🔇 Audio deshabilitado");
        } else {
            println!("🔊 Audio habilitado");
            let _ = self.play_background_music();
        }
    }
    
    pub fn set_volume(&self, volume: f32) {
        if let Ok(sink_guard) = self.music_sink.lock() {
            if let Some(ref sink) = *sink_guard {
                sink.set_volume(volume.clamp(0.0, 1.0));
            }
        }
    }
    
    pub fn handle_gamepad_controls(&mut self, gilrs: &Gilrs) -> Result<(), Box<dyn std::error::Error>> {
        // Check for gamepad button presses for audio controls
        for (_id, gamepad) in gilrs.gamepads() {
            // Right shoulder button (RB/R1) for next track
            if gamepad.is_pressed(Button::RightTrigger2) {
                return self.next_track();
            }
            
            // Left shoulder button (LB/L1) for previous track
            if gamepad.is_pressed(Button::LeftTrigger2) {
                return self.previous_track();
            }
            
            // Start button for toggle audio
            if gamepad.is_pressed(Button::Start) {
                self.toggle_audio();
                return Ok(());
            }
            
            // Select/Back button for volume control (could implement volume adjustment)
            if gamepad.is_pressed(Button::Select) {
                // Could cycle through volume levels here
                self.set_volume(0.5); // Set to medium volume
                println!("🎮 Volume set to 50%");
                return Ok(());
            }
        }
        
        Ok(())
    }
    
    fn print_song_info(&self, track_index: usize) {
        match track_index {
            0 => println!("   🎤 \"Energía positiva y ritmo contagioso\" 🎤"),
            1 => println!("   🎤 \"Melodía misteriosa y cautivadora\" 🎤"),
            2 => println!("   🎤 \"Introspección y honestidad emocional\" 🎤"),
            3 => println!("   🎤 \"Historia de amor atemporal\" 🎤"),
            4 => println!("   🎤 \"Determinación y empoderamiento\" 🎤"),
            _ => println!("   🎤 \"Taylor Swift cantando para ti...\" 🎤"),
        }
    }
}
