#!/usr/bin/env python3
"""
Generate .vplay level files for Pang game based on the current configuration
"""

import json
import os

# Data from main.vpy
locations = [
    {"name": "01_mount_fuji", "display": "MOUNT FUJI (JP)", "bg": "fuji_bg", "x": 40, "y": 110},
    {"name": "02_mount_keirin", "display": "MOUNT KEIRIN (CN)", "bg": "keirin_bg", "x": 40, "y": 79},
    {"name": "03_emerald_buddha", "display": "EMERALD BUDDHA TEMPLE (TH)", "bg": "buddha_bg", "x": -40, "y": -20},
    {"name": "04_angkor_wat", "display": "ANGKOR WAT (KH)", "bg": "angkor_bg", "x": -10, "y": 10},
    {"name": "05_ayers_rock", "display": "AYERS ROCK (AU)", "bg": "ayers_bg", "x": 20, "y": 40},
    {"name": "06_taj_mahal", "display": "TAJ MAHAL (IN)", "bg": "taj_bg", "x": 50, "y": 70},
    {"name": "07_leningrad", "display": "LENINGRAD (RU)", "bg": "leningrad_bg", "x": 80, "y": 100},
    {"name": "08_paris", "display": "PARIS (FR)", "bg": "paris_bg", "x": -85, "y": -40},
    {"name": "09_london", "display": "LONDON (UK)", "bg": "london_bg", "x": -50, "y": -10},
    {"name": "10_barcelona", "display": "BARCELONA (ES)", "bg": "barcelona_bg", "x": -15, "y": 30},
    {"name": "11_athens", "display": "ATHENS (GR)", "bg": "athens_bg", "x": 15, "y": 60},
    {"name": "12_pyramids", "display": "PYRAMIDS (EG)", "bg": "pyramids_bg", "x": 50, "y": 90},
    {"name": "13_kilimanjaro", "display": "MOUNT KILIMANJARO (TZ)", "bg": "kilimanjaro_bg", "x": 85, "y": 20},
    {"name": "14_new_york", "display": "NEW YORK (US)", "bg": "newyork_bg", "x": -90, "y": 50},
    {"name": "15_mayan_ruins", "display": "MAYAN RUINS (MX)", "bg": "mayan_bg", "x": -45, "y": 0},
    {"name": "16_antarctica", "display": "ANTARCTICA (AQ)", "bg": "antarctica_bg", "x": 0, "y": -60},
    {"name": "17_easter_island", "display": "EASTER ISLAND (CL)", "bg": "easter_bg", "x": 45, "y": -30},
]

enemy_counts = [1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7]
enemy_speeds = [1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5]

difficulties = ["easy", "easy", "easy", "medium", "medium", "medium", "medium", 
                "medium", "hard", "hard", "hard", "hard", "very_hard", "very_hard", 
                "very_hard", "extreme", "extreme"]

def create_level(idx, loc):
    """Create a .vplay level file"""
    level_num = idx + 1
    
    level = {
        "version": "2.0",
        "type": "level",
        "metadata": {
            "name": loc["name"],
            "displayName": loc["display"],
            "author": "Pang Team",
            "difficulty": difficulties[idx],
            "levelNumber": level_num,
            "mapPosition": {
                "x": loc["x"],
                "y": loc["y"]
            }
        },
        "worldBounds": {
            "xMin": -96,
            "xMax": 95,
            "yMin": -128,
            "yMax": 127
        },
        "gameConfig": {
            "enemyCount": enemy_counts[idx],
            "enemySpeed": enemy_speeds[idx],
            "timeLimit": 0
        },
        "layers": {
            "background": [
                {
                    "id": f"{loc['bg']}_bg",
                    "type": "background",
                    "vectorName": loc["bg"],
                    "x": 0,
                    "y": 0,
                    "rotation": 0,
                    "scale": 1,
                    "physicsEnabled": False,
                    "layer": "background"
                }
            ],
            "gameplay": []
        }
    }
    
    return level

def main():
    # Create output directory
    output_dir = "assets/levels"
    os.makedirs(output_dir, exist_ok=True)
    
    # Generate all 17 levels
    for idx, loc in enumerate(locations):
        level = create_level(idx, loc)
        filename = os.path.join(output_dir, f"{loc['name']}.vplay")
        
        with open(filename, 'w') as f:
            json.dump(level, f, indent=2)
        
        print(f"✓ Created {filename}")
    
    print(f"\n✓ Generated {len(locations)} level files")

if __name__ == "__main__":
    main()
