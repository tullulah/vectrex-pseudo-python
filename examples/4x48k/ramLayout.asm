; constants
;delayResetSp        EQU      1                            ; how long to delay between the animations of the spawn
maxVaultEnemy       EQU      3                            
instLettBrt         EQU      #85                          ; brightness of the instruction text during attract mode
delayReset          EQU      3                            ; how long to delay between switching sprites in animating the running or climbing Tom
explScale1          EQU      #$94                         ;
explScale2          EQU      28
fuelFillUp          EQU      #40                          ; when a player respawns or starts the game, this is how much fuel he gets
FuelDelay           EQU      34                           ; how long to wait before removing a unit from the Fuel bar
MaxBits             EQU      4                            ; maximum # of bits allowed on screen at a time
maxGunShots         EQU     10                            ; when Tom picks up Gun, this is how many shots he gets
MaxPBulls           EQU      2                            ; maximum # of player bullets allowed on Platform levels
MAX_TEXT_HEIGHT     EQU      $FC                          ; used for Game Over screen
maxEnMissH          EQU      #133
MaxMSExpl           EQU      3                            ; maximum # of minestorm explosions allowed on screen at once
RecogDelay          EQU      2   
tomYLoc             EQU     #-100 ; #-124                 ; y-coordinate location of Tom's space ship
showScDuration      EQU     #150                          ; how long to shw the score
spaceScale          EQU     #148                          ; the scale (for MY_MOVE_TO_D) for player ship and enemy bullets
speedUp1            EQU     #6                            ; what WaveNum the game starts to go faster overall (play movments, enemy movements, shots, etc)
speedUp2            EQU     #9                            ; what WaveNum the game starts to go faster overall (play movments, enemy movements, shots, etc)
inActive            EQU     #126                          ; used in platform to designate in-active enemies and in-active bullets
msExplSizeLg        equ     #50                           ; the large size of the minestorm explosion
msExplSizeMed       equ     #30                           ; the medium size of the minestorm explosion
msExplSizeSm        equ     #10                           ; the small size of the minestorm explosion
minestormExplosionPattern  =  $eeba 
minestorm_move_y_draw_x  =   $ea7f 
; !!!!!!!! PUT ABOVE IN ALPHABETICAL ORDER!!!!!!



; C880-CBEA is available ram on the VECTREX
; ******************* R A M   S y m b o l s  ( v a r i a b l e s )
; -------------------
; IN GAME RAM 
; -------------------

                    bss
                    org      $c880
;statusLineRAM       ds       23                           ; a string containing the remaining ships and the player's score 
PlayerSc            ds       7                            ; player's score and the hi score are stored in these 7 bytes!
;HiScore             ds       7                            ; the highest score obtained so far
sfx_pointer         ds       2                            ; voice #1
sfx_pointer2        ds       2                            ; voice #2
sfx_pointer3        ds       2                            ; voice #3
sfx_status          ds       1                            ; next three are for sounds on voice #, 2, 3
sfx_status2         ds       1
sfx_status3         ds       1
BackGndCtr          ds       1                            ; counter for the background sound playing on voice #2 during space shooter
BkGndCtEnd          ds       1  
TomVisible          ds       1                            ; = 0 , then Tom isn't on the screen, = 1, then he is
tomY                ds       1                            ; location of Tom's space ship or Tom himself on platforms
tomX                ds       1
TomLastDir          ds       1
TomActionData       ds       1                            ; see below - this just shows which region Tom is in
; bits 1-4    :  What region Tom is currently in (1-3)
; bits 5 - 8  :  open
TomDirData          ds       1
; bit 1 :  if = 1 then Tom is standing still, facing left
; bit 2 :  if = 1 then Tom is standing still, facing right
; bit 3 :  if = 1 then Tom is running left
; bit 4 :  if = 1 then Tom is running right
; bit 5 :  if = 1 then Tom is ducking while pointed left
; bit 6 :  if = 1 then Tom is ducking while pointed right
; bit 7 :  if = 1 then Tom is climbing the ladder
; bit 8 :  if = 0 then Tom is dead
; if bit 8 and bit 5 are set (total of 9), then Tom ran out of energy.
ThrustCtr           ds       1
; starsDim take from $c890 - $c8ad
;starsDim            ds       30                          ; 30 bytes - these are the x,y locations for dimmer stars
                                                          ; 15 stars total, so 30 are needed for each x,y location
; starsBright take from $c8ae - $c8bf
;starsBright         ds       16                          ; 16 bytes - these are the x,y locations for brighter stars
                                                          ; 8 stars total, so 16 are needed for each x,y location
secondStars         ds       1
starsNmi            ds       1                            ; used to draw stars every other frame
StarCount           ds       1                            ; used for looping through scrolling stars
DropFExpY           ds       1
DropFExpX           ds       1
DropFuelY           ds       1
DropFuelY2          ds       1
DropFuelX           ds       1
DropFuelX2          ds       1
DropFExpCtr         ds       1                            ; how long to keep drop fuel explosion on screen
; **** begin PowerUp stuff
scale1              ds       1
scale1_2            ds       1
IconYCoord          ds       1
nmi2                ds       1
BTN3Psd             ds       1
PowerUpNmi          ds       1
PUTextBrt           ds       1
PUTextY             ds       1
PUTextX             ds       1
;PUBarXStart         ds       1                           ; NOT NEEDED: 9-21-24
PowerUpTXT          ds       2                            ; pointer to "SHIELDS" or "GUIDED MISSILE"
PUBarXCtr           ds       1
PowerUp             ds       1                            ; if = 1 then a power up is falling in place of drop fuel
PowerUp2            ds       1                            ; if = 1 then a power up is falling in place of drop fuel
PowerUpType         ds       1                            ; if != 0 then space ship has a power up! (1=shields, 2=guided missile)
PowerUpTypeSav      ds       1                            ; this is when the player carries over a power up to Vault
                                                          ; and now he's back in SS...   so load it back in
ShieldOn            ds       1
ShieldCtr           ds       1                            ; every 5 reductions of the the shield brightess, we reduce the shots bar (strength bar)
SHbright            ds       1                            ; how bright the shield currently is
ShowScoreCtr        ds       1                            ; show the score and ships remaining if this is > 0
DropFOnScr          ds       1                            ; how many drop fuels are on screen (fuel or power ups)
DropFCntDn          ds       1                            ; this counts down until it's time to put another drop fuel on screen
DropFNmi            ds       1
DropFFlipped        ds       1                            ; if = 1 then then the power up has alredy been flipped back to Drop Fuel
DropFFlipped2       ds       1                            ; if = 1 then then the power up has alredy been flipped back to Drop Fuel
DropFSpr            ds       2
DropFSpr2           ds       2
LowFuelSndCtr       ds       1                            ; used to get it to play the low_fuel_snd 2 times in a row, then back to back ground sounds
CurMissY_s          ds       1                            ; these two are used to compensate for the shields being on
CurMissX_s          ds       1
; **** end PowerUp stuff

; **** begin intermission stuff
barrierBright       ds       1                            ; if > 0 then we need to start drawing the barrier
doorYPos            ds       1                            ; used for when the space ship door swings open
doorXPos            ds       1                            ; used for when the space ship door swings open
interSection        ds       1                            ; each intermission can have multiple sections
shipFlyPtr          ds       1                            ; points to where we are at in shipFlyPattern, if = #255, then it’s overwith
shipFlyPauseCtr     ds       1                            ; where we are in current ship pause
shipFlyThrPtr       ds       1                            ; 0 = normal size thrust, 1 = medium size thrust, 2 = BIG thrust (when Ship is going up)
shipFlyPattern      ds       2                            ; points to our movement table: shipFlyPatternX:  where x = 1,2,3,4,5
shipFlyPatRpt       ds       1                            ; for current movement row, how many times do we process this same row?
shipDoorOpening     ds       1                            ; if > 0 then we’re opening the door - this represent the door height
tomMovePattern      ds       2                            ; points to our movement table: tomMovePatternX:  where x = 1,2,3,4,5
tomMovePtr          ds       1                            ; points to where we are at in tomMovePattern, if = #255, then it’s overwith
tomMovePauseCtr     ds       1                            ; where we are in current tom movement pause
; **** end intermission stuff

; **** begin enemy collision det padding ****
Height              ds       1 
HeightSC            ds       1     
Width               ds       1
padY                ds       1
padX                ds       1
EnShColW_1          ds       1                            ; width for top of player ship to enemy
EnShColW_2          ds       1                            ; width for middle of player ship to enemy
EnShColW_3          ds       1                            ; width for bottom of player ship to enemy
EnShColPadY         ds       1                            ; how much to pad enemy y coordinate (+ or -)
EnShColPadX         ds       1                            ; how much to pad enemy x coordinate (+ or -)
EnShColMYC          ds       1                            ; the divider of the y-coordinate of the middle of player's ship during coll det with enemy
; **** end enemy collision det padding ****

; **** begin enemy missile y and x pad ****
MissYPad            ds       1                            ; when putting enemy missile on screen, how much to add or sub to y-coord
MissXPad            ds       1                            ; "  "  "  how much to add or sub to x-coord
ReverseEW           ds       1                            ; if = 1 then if the enemy is too far towards the edge of screen and it's trying to shoot east or west, it will reverse the direction of bullet
; **** begin enemy missile y and x pad ****

footStepsCtr        ds       1                            ; counter in between Tom's footsteps sound being played
FinalCountDn        ds       1                            ; countdown delay for end of space shooter wave
FinalExitY          ds       1                            ; this is the y-coordinate for the final exit ladder - when Tom gets to this, he's completed this platform
OnExitLad           ds       1                            ; 0 = not on exit ladder, 1 = on exit ladder at bottom of screen, 2 = exit ladder on top of screen
FuelUpSound         ds       1                            ; a counter to play the Fuel Picked Up sound effect
FuelFY              ds       1                            ; y-coordinate of "F" for Fuel #1
FuelFX              ds       1                            ; x-coordinate of "F" for Fuel #1
FuelFY2             ds       1                            ; y-coordinate of "F" for Fuel #2
FuelFX2             ds       1                            ; x-coordinate of "F" for Fuel #2
FuelPods            ds       1                            ; total # of "F" fuel pods that will be shown on current platform
Fuel3               ds       1
FuelCtr             ds       1                            ; holds the countdown for the next removal of a single Fuel unit in fuel bar 
FuelWarnCtr         ds       1                            ; this hold the duration of the fuel low warning sound 
delayCounter        ds       1
BirdSnd             ds       1                            ; counter in-between the bird flapping sounds (when Bird is on screen)
Counter             ds       1                            ; just a generic counter variable
Counter2            ds       1                            ; just a generic counter variable
ExitLadScale        ds       1                            ; (platform) will hold the scale value of the exit ladder
;ExplScale           ds       1                            ; the scale used for MY_MOVE_TO_D on either space shooter or platform
expl1Y              ds       1                            ; explosion piece 1 y coord 
expl2Y              ds       1                            ; explosion piece 2 y coord 
expl3Y              ds       1                            ; explosion piece 3 y coord 
expl4Y              ds       1                            ; explosion piece 4 y coord 
expl1X              ds       1                            ; explosion piece 1 x coord 
expl2X              ds       1                            ; explosion piece 2 x coord 
expl3X              ds       1                            ; explosion piece 3 x coord 
expl4X              ds       1                            ; explosion piece 4 x coord
explosionSize       ds       1                            ; has the size of the Mine Storm explosion for current SS wave/subwave
explodeNmi          ds       1
ExplosionDR         ds       1                            ; explosion counter delay reset value
ExplosionDC         ds       1
ExplosionAC         ds       2                            ; explosion pieces animation counter
ExplosionAC1        ds       1                            ; explosion animattion counter for each of the 4 explosion pieces
ExplosionAC2        ds       1
ExplosionAC3        ds       1
ExplosionAC4        ds       1
ExplosionDC1        ds       1                            ; piece one's delay in animating the different rotation shapes
ExplosionDC2        ds       1
ExplosionDC3        ds       1
ExplosionDC4        ds       1
explosion1Y         ds       1                            ; y-coord of location of explosion #1
explosion1X         ds       1                            ; x-coord of location of explosion #1
explosion2Y         ds       1                            ; y-coord of location of explosion #2
explosion2X         ds       1                            ; x-coord of location of explosion #2
explosionFlags      ds       1                            ; this is the animating exploding pieces
; bit 1 : 0 = explosion 1 is open, 1 = explosion 1 is in use
; bit 2 : 0 = explosion 2 is open, 1 = explosion 2 is in use
; bit 3 : 0 = explosion 3 is open, 1 = explosion 3 is in use
; bit 4 : 0 = explosion #1 is using first explosion sprites, 1 = using second explosion sprites
; bit 5 : 0 = explosion #2 is using first explosion sprites, 1 = using second explosion sprites
; bit 6 : 0 = explosion #3 is using first explosion sprites, 1 = using second explosion sprites
; bit 7 :
; bit 8 :
;explFlags           ds       1                            ; how many minestorm explosions are happening currently
explBrightP         ds       1
explSound           ds       1
explScale           ds       1                            ; used to scale the "new" Minestorm explosions
nmiMove             ds       1
nmiMoveRes          ds       1
nmiNewCnt           ds       1                            ; just another nmi count as of 9-19-24. Can be used for anything.
MTCountDown         ds       2                            ; Major Tom title screen main countdown (16-bit countdown timer of 10 seconds)
;MTSecs              ds       1                            ; keeps track of total # of seconds we are on title screen
MTTotalFr           ds       1
MTBright            ds       1                            ; brightness for MAJOR TOM title screen vector
MTScale             ds       1                            ; scale for MAJOR TOM title screen vector
msExpl1             ds       9                            ; attributes for the new Minestorm Explosions #1
msExpl2             ds       9                            ; attributes for the new Minestorm Explosions #2
msExpl3             ds       9                            ; attributes for the new Minestorm Explosions #3
;msExpl1Y            ds       1                            ; y-coord of mine storm explosion #1 
;msExpl2Y            ds       1                            ; y-coord of mine storm explosion #2 
;msExpl3Y            ds       1                            ; y-coord of mine storm explosion #3 
;msExpl1X            ds       1                            ; x-coord of mine storm explosion #1 
;msExpl2X            ds       1                            ; x-coord of mine storm explosion #2 
;msExpl3X            ds       1                            ; x-coord of mine storm explosion #3
MSE2Ctr             ds       1                            ; store the counter that is used while minestorm explosion sound is playing
DeSpawnCtr          ds       1
DeathCountDown      ds       1                            ; if this has a value, then player has died and this is the count down before respawning
debugFlag           ds       1                            ; used to flag stuff for temp debuggins
debug_0             ds       1                            ; used for reg_to_decimal
debug_1             ds       1                            ; used for reg_to_decimal
debug_2             ds       1                            ; used for reg_to_decimal
debug_3             ds       1                            ; used for reg_to_decimal
debug_4             ds       1                            ; used for reg_to_decimal
dotScale            ds       1
decScale            ds       1
decScale2           ds       1
decScale3           ds       1
bright              ds       1
bright2             ds       1
bright3             ds       1
BitsOnScr           ds       1                            ; how many bits are currently on screen / which bits are picked up!
BitsDir             ds       1                            ; direction each bit is going (0=left, 1=right)
BitsDir2            ds       1
BitsDir3            ds       1
BitsDir4            ds       1
scale               ds       1
scale2              ds       1
scale3              ds       1
GameScreen          ds       1                            ; tells where we are in the game (title, bonus, game over, platform, space shooter, etc)
GameOverCtr         ds       1                            ; How long the game over song jingle plays
ButtonPressed       ds       1                            ; the last button player has pressed
TitleScrnCtr        ds       1                            ; a generic countdown timer for an intermission screen
TitleSngCtr         ds       2                            ; counter to delay in-between playing Space Oddity title song
Tmp                 ds       1                            ; the next 5 are free to use anywhere
Tmp2                ds       1
Tmp3                ds       1
Tmp4                ds       1
Tmp5                ds       1
Tmp6                ds       1
Tmp7                ds       1
Tmp8                ds       1
ThreeSprPause       ds       1
DoSoundPtr          ds       2                            ; a pointer to a musical definition for DoSound
DoSoundCtr          ds       1                            ; if a DoSound is playing, this is the countdown for it
GameSpeed           ds       1                            ; how fast everything moves in the game. Starting with Wave #7, it goes 25% faster, after wave #10, it goes 50% faster
GunShots            ds       1                            ; how many gun shots Tom has on the gun he picked up
GunBitBrt           ds       1
JumpDir             ds       1
JumpProg            ds       1                            ; where we are in the jump sequence of good ole Tom
WaveEnOnScr         ds       1                            ; this holds how many enemies are currently on the screen (SS only)
InBTCounter         ds       1                            ; "In Between" counter - a countdown between waves
Inertia             ds       1
InertiaCD           ds       1
introWaveNum        ds       1                            ; this is used for intro to Space Shooter: "Wave 1", etc
initEnCall          ds       1                            ; set to 1 when initEnemies is called
EnRptTable          ds       3                            ; when we encounter an 11 in table, we repeat a certain # times - this keeps track of how many times we've done it thus far
EnemyYHold          ds       1                            ; next two are where to place enemy AFTER enemy spawn completes
EnemyXHold          ds       1
EnemyType           ds       1                            ; for each of the "live" enemies, this is the type of enemy
EnemyType2          ds       1
EnemyType3          ds       1
; 1 = Machine Champion
; 2 = Machine Piper
; 3 = Machine Bird
; 4 = Machine Star
; 5 = Machine Bat
EnemyInfo1          ds       1                            ; defines the behavior of enemy sprite #1-#5
EnemyInfo2          ds       1
EnemyInfo3          ds       1
EnemyInfo4          ds       1
EnemyInfo5          ds       1
; Bit 5-8 : points to a shape/movement array (1 = eSprite1, 2 = eSprite2,...)
; Bit 1-4 : value is 1-15 and points to a sprite array of shapes ( 1 = eSprite1, 2 = eSprite2...)
EnemyHealth         ds       1                            ; each bit represents something about each enemy
; Bit 1: 1 = enemy #1 is alive
; Bit 2: 1 = enemy #2 is alive
; Bit 3: 1 = enemy #3 is alive
; Bit 4: 1 = enemy #4 is alive
; Bit 5: 1 = enemy #5 is alive
; Bit 6: 1 = enemy #6 is alive
; Bit 7: 1 = enemy #7 is alive
; Bit 8: 1 = enemy #8 is alive
EnemyHealth2        ds       1
; Bit 1: 1 = enemy #9  is alive
; Bit 2: 1 = enemy #10 is alive
; Bit 3: 1 = enemy #11 is alive
; Bit 4: 1 = enemy #12 is alive
; Bit 5: 1 = enemy #13 is alive
; Bit 6: 1 = enemy #14 is alive
; Bit 7: 1 = enemy #15 is alive
; Bit 8: 1 = enemy #16 is alive
EnemySpeed          ds       1
EnemySpeed2         ds       1
EnemySpeed3         ds       1
EnemySpeed4         ds       1
EnemySpeed5         ds       1
EnMoveTbl1          ds       10                           ; the movement table for each sprite (pointer to a rom location)
EnSprList1          ds       10                           ; points to a sprite table of shapes (1 = eSprite1List, 2 = eSprite2List...)
EnDuplCtr           ds       5                            ; the # of times to repeat the movement pattern for enemy #1-#5
EnCollH             ds       1                            ; this is how much to extend the height of the enemy collision detections box (used for the later phases when player is shooting so fast and enemies are moving to fast)
EnSpeedCtr          ds       1                            ; how fast enemy #1-5 moves (higher number = slower movements) 
EnSpeedCtr2         ds       1
EnSpeedCtr3         ds       1
EnSpeedCtr4         ds       1
EnSpeedCtr5         ds       1
;EnLastShoot         ds       1                            ; the enemy that last shot a missile
EnBirdOnScr         ds       1                            ; if = 1 then bird is on screen, = 0 otherwise
Enemy1Pos           ds       5                            ; where in the movement array the enemy (#1-5) is currently at (255=attacking mode?)
Enemy1Y             ds       1                            ; y-coordinate of enemy #1-#15
Enemy2Y             ds       1                            ; y-coordinate of enemy #1-#15
Enemy3Y             ds       1                            ; y-coordinate of enemy #1-#15 
Enemy4Y             ds       1                            ; y-coordinate of enemy #1-#15 
Enemy5Y             ds       1                            ; y-coordinate of enemy #1-#15 
Enemy6Y             ds       1                            ; y-coordinate of enemy #1-#15 
Enemy7Y             ds       1                            ; y-coordinate of enemy #1-#15 
Enemy8Y             ds       1                            ; y-coordinate of enemy #1-#15 
Enemy9Y             ds       1                            ; y-coordinate of enemy #1-#15 
Enemy10Y            ds       1                            ; y-coordinate of enemy #1-#15 
Enemy11Y            ds       1                            ; y-coordinate of enemy #1-#15 
Enemy12Y            ds       1                            ; y-coordinate of enemy #1-#15 
Enemy13Y            ds       1                            ; y-coordinate of enemy #1-#15 
Enemy14Y            ds       1                            ; y-coordinate of enemy #1-#15 
Enemy15Y            ds       1                            ; y-coordinate of enemy #1-#15 
Enemy1X             ds       1                            ; x-coordinate of enemy #1-#5 
Enemy2X             ds       1                            ; x-coordinate of enemy #1-#5 
Enemy3X             ds       1                            ; x-coordinate of enemy #1-#5 
Enemy4X             ds       1                            ; x-coordinate of enemy #1-#5 
Enemy5X             ds       1                            ; x-coordinate of enemy #1-#5 
Enemy6X             ds       1                            ; x-coordinate of enemy #1-#5 
Enemy7X             ds       1                            ; x-coordinate of enemy #1-#5 
Enemy8X             ds       1                            ; x-coordinate of enemy #1-#5 
Enemy9X             ds       1                            ; x-coordinate of enemy #1-#5 
Enemy10X            ds       1                            ; x-coordinate of enemy #1-#5 
Enemy11X            ds       1                            ; x-coordinate of enemy #1-#5 
Enemy12X            ds       1                            ; x-coordinate of enemy #1-#5 
Enemy13X            ds       1                            ; x-coordinate of enemy #1-#5 
Enemy14X            ds       1                            ; x-coordinate of enemy #1-#5 
Enemy15X            ds       1                            ; x-coordinate of enemy #1-#5
EnemyM1Ctr          ds       5                            ; this is a countdown of screen frames in between missile movements (it's the speed of missiles)
EnemyMiss1Y         ds       1
EnemyMiss2Y         ds       1
EnemyMiss3Y         ds       1
EnemyMiss4Y         ds       1
EnemyMiss5Y         ds       1
EnemyMiss1X         ds       1
EnemyMiss2X         ds       1
EnemyMiss3X         ds       1
EnemyMiss4X         ds       1
EnemyMiss5X         ds       1
;EnMissCutOff        ds       1                            ; whatever is stored in this var will have the cut off for # of enemy missile to draw on current frame. It depends on WaveMaxBull. If = 4, then this var will be 2.
EnemyReplen         ds       1                            ; if = 1 then the enemy was replenished immediately after being shot
EnemyM1Info         ds       5                            ; enemy #1-#5 missile attributes (see below) 
; bit  1   = 1 then we're using the straight down sprite
; bit  2   = 1 then we're using the left diagonal sprite
; bit  3   = 1 then we're using the right diagonal sprite
; bit  4   = 1 then we're using left or right sprite (due west or due east)
; bits 5-8 = the speed of the missile (smaller number makes it move faster)
Phase2Ctr           ds       1                            ; store the counter that is used while recognizer to phase two sound effect
PlBullCount         ds       1                            ; this will hold the total # of player bullets that are active (eg: if = 2 then there are two player bullets on screen)
plBullSpeed         ds       1                            ; how fast the player's bullet will go (depends on what WaveNum he is on)
;PlatItems           ds       2                           ; this has the status of all "items" Tom can pick up
; bit 1  = if gun #1 is active then 1, else 0
; bit 2  = if gun #2 is active then 1, else 0
; bit 3  = if power up #1 is active then 1, else 0
; bit 4  = if piece #1 is active
; bit 5  = if piece #2 is active
; bit 6  = if piece #3 is active
; bit 7  = if piece #4 is active
; bit 8  = if piece #5 is active
; bit 9  = if piece #6 is active
; bit 10 = if piece #7 is active
; bit 11 = if Fuel is active
PlatEnOS            ds       1                            ; if = 1 then platform enemy is on screen
PlatLocY            ds       1                            ; y location to move to in orfer to draw the platform
PlatLocX            ds       1                            ; x location to move to in orfer to draw the platform
PlatLocY2           ds       1                            ; .. ... ... draw the second half of the platform
PlatLocX2           ds       1                            ; .. ... ... draw the second half of the platform
PlatScale           ds       1                            ; scale used to draw the current level's platform
PlatExitLad         ds       2                            ; this will hold the table of values for the exit ladder (shown when bits are all collected)
PlatCompCtr         ds       1                            ; holds the time to show the Platform complete bonus screen
PlatData            ds       2                            ; this will hold the address location of the platform data (y,x coords of barriers, enemy robots, diamonds, etc)
PlatLads            ds       2                            ; this will hold the address location of the ladder data for each platform level
Plat_R1             ds       2                            ; Region 1 of current platform
Plat_R2             ds       2                            ; Region 2 of current platform
Plat_R3             ds       2                            ; Region 3 of current platform
PlatFlags           ds       1
PlatPreInit         ds       1                            ; = 0 if the Platform Pre-init has not happened, = 1 if it has      
PlatMinX            ds       1                            ; these next two are max left and max right Tom can go to
PlatMaxX            ds       1
PlatGuns            ds       2                            ; this will hold the address location of the gun data for each platform level
PlatGunX1           ds       1
PlatGunX2           ds       1
PlatGunY1           ds       1
PlatGunY2           ds       1
PlatLevel           ds       1                            ; which platform level Tom is on (1-10...I hope we can get to 10)
PlatNmMvTbl         ds       3                            ; for a given enemy (in vault) this will hold the number of "stacked"
                                                          ; (overflow) movement tables the active alien has defined (for longer
                                                          ; patterns)
PlatMvSpNum         ds       3                            ; if the movement table spans across multiples, 
                                                          ; this is the number (0,1,2) we are currently on
PlatMvNum           ds       3                            ; for a given vault enemy, this is the pointer to the current sprite movement table (ex: #10 = platSprite11)
PlatMvNumH          ds       1                            ; this holds the value for PlatMvNum while a spawn is happening
PlatNmMvTblH        ds       1                            ; this holds the value for PlatNmMvTbl while a spawn is happening
PlatMvSpNumH        ds       1                            ; this holds the value for PlatMvSpNum while a spawn is happening
CurrEnSCtrH         ds       1
PlatMovTblH         ds       2                            ; while spawn is happening, this holds PlatMovTbl
PlatMovTbl          ds       10                           ; for any enemy that is on screen for a Platform level, this points to his movement table (there are some movement tables that span across as many as 5 tables of 127 each)
ShrStarted          ds       1                            ; if nonzero, thern the shrking of platform playfield ha begun
ShrDelay            ds       1                            ; during the shrinking of platform sequence, this is a little dealy in b/t each
SubWaveNum          ds       1                            ; which "sub" wave we're on in a Wave for space shooter
WaveChaseSp         ds       1                            ; how fast the chase enemies fly towards player ship
WaveChaseSr         ds       1                            ; pointer to ChaseSpr1,2,3,4 for the sprite (vector list) shapes for the chase enemy
WaveMaxBull         ds       1                            ; maximum # of enemy bullets allowed on screen at once
WaveMaxEn           ds       1                            ; holds the maximum number aliens allowed on screen at a time for current wave/subwavenum
WaveNumKill         ds       1                            ; holds the number of enemies remaining that need to be killed to end the wave
WavSkipStrs         ds       1                            ; if = 1 then on this WaveNum / SubWaveNum, we suppress drawing the stars to gain some cycles back
WaveNumStr          ds       7                            ; used to show what wave # player is starting in on
IncMY               ds       1                            ; (in the loop) how much to increment Y of the current MISSILE
IncMX               ds       1                            ; (in the loop) how much to increment X of the current MISSILE
incTX               ds       1                            ; used in processJump to temp store tom's x coord
incTY               ds       1                            ; used in processJump to temp store tom's x coord
CurrBckGndSnd       ds       2                            ; (space shooter) each each has a different background sound - this points to the sound definition (Arkos Tracker)
CurrPlatPF          ds       2                            ; this will hold the address of the Platform playfield definition for the PlatLevel we are on
CurrPlatPF2         ds       2                            ; second half of platform
CurrPlatRules       ds       2                            ; for current platform, this points to the "rules" table for enemies to enter screen
RulesInAction       ds       1                            ; the rule # of each of the 3 possible enemies on screen are stored here. It's to stop the same rule from getting tripped more than once.
RulesInAction2      ds       1
RulesInAction3      ds       1
LstRIAIndex         ds       1                            ; this holds the last index inserted into RulesInAction array
LstEnemyType        ds       1                            ; this holds the EnemyType of last enemy put on screen
CurrMissInf         ds       1                            ; will hold EnemyM1Info - EnemyM5Info
CurrLevel           ds       1                            ; whatever level we are on regardless if it's space or platform
CurMissY            ds       1
CurMissX            ds       1
CurMissSpr          ds       1
CurMissSCr          ds       1                            ; missile speed
CurrEnShoot         ds       1                            ; this is read from enemy movement table and if = 1, enemy will fire missile
CurrIncY            ds       1                            ; (in the loop) how much to increment enemy y-coordinate
CurrIncX            ds       1                            ; (in the loop) how much to increment enemy x-coordinate
CurrEnPos           ds       1                            ; (in the loop) will hold where enemy is in the movements table 
CurrEnSCtr          ds       1                            ; (in the loop) the speed of the enemy 
CurrDupCtr          ds       1                            ; (in the loop) where in the movement table's duplicate counter current enemy is at
CurrMSprite         ds       1                            ; (in the loop) 0 = don't fire, any non-zero # is which missile to fire
runNmiP             ds       1                            ; used to time the jumping sprite animations
runNmi              ds       1                            ; used to time the animations in Tom running sprites
Recog1Info          ds       1                            ; attributes that describe Recognizer #1
; bit 1 : if set to 1 = Recognizer #1 is active 
; bit 2 : if set to 1 = Recognizer #1 is moving left  (x = x - 1)
; bit 3 : if set to 1 = Recognizer #1 is moving right (x = x + 1)
; bit 4 : if set to 1 = Recognizer #1 is moving up    (y = y + 1)
; bit 5 : if set to 1 = Recognizer #1 is moving down  (y = y - 1)
; bit 6 : if set = 0 then recognizer is in phase #1 (he's on the outer edges waiting to be on same x- axis as player or same y-axis of player)
;         if set = 1 then recognizer is in phase #2 (he's charging towards player and moving much faster)
Recog2Info          ds       1                            ; attributes that describe Recognizer #2
Recog3Info          ds       1                            ; attributes that describe Recognizer #3
RecogPause          ds       1                            ; how long to wait until putting another recognizer onto screen (set in initPlatform)
RecogPCtr           ds       1                            ; current position in the RecogPause countdown timer
RecogSpeed          ds       1                            ; how long to wait until any recognizers on screen move (set in initPlatform)
RecogSpdCtr         ds       1                            ; current position in the speed countdown
RecogOnScr          ds       1                            ; total # of Recogs currently on screen
RecogAnimCtr        ds       1                            ; used in selecting the different Recognizer sprites for animation effect
RecogCounter        ds       1                            ; current position in RecogAnimCtr for recorgnizer sprites also used in Space Shooter part for countdown of Drop Fuel
RecogX1             ds       1
RecogX2             ds       1
RecogX3             ds       1
RecogY1             ds       1
RecogY2             ds       1
RecogY3             ds       1
RecogNmFlag         ds       1                            ; can be 0,8,9   if 8 or 9 we add another recog to screen after fuirst recog is detsroyed
;timerUsage          ds       2
LadYStart           ds       1                            ; these next two are set for the current ladder Tom is on
LadYEnd             ds       1
LastSubWave         ds       1                            ; hold the # of the last SubWaveNum for this Wave (ex: Wave 1 has 4 sub waves)
isTinySquad         ds       1                            ; if = 1 then we are on a wave that has boulders or other small squadron enemies
SquadMoveTbl        ds       2                            ; pointer to the one sprite movement table for Squadron sprites
SquadSprite         ds       2                            ; pointer to the one sprite shape group for Squadron sprites
SquadCollDet        ds       1                            ; for WaveType=9, this value is used to determine how to do collision detection b/t squad and player's bullet
SquadShSpeed        ds       15                           ; for WaveType=9, each enemy has a speed his bullets will go
                                                          ; (0=slow, 1=medium, 3 = fast)
WarpTxtCtr          ds       1                            ; duration of "Warp!" text being on the screen
WarpTxtY            ds       1                            ; y-coord location of the "Warp!" text
WarpTxtX            ds       1                            ; x-coord location of the "Warp!" text
WarpTxtBrt          ds       1                            ; brightness of the "Warp!" text
WarpSpeed           ds       1                            ; if > 0 then we're in warp mode (Astro Blaster)
WarpSecCtr          ds       1                            ; (space shooter) just counts from 49 to 0 - used for the enemy bullet slowdown
WavePutOnScr        ds       1                            ; how many enemies have been placed on the screen so far from Squadron
WaveNum             ds       1
WaveNxtEnNm         ds       1                            ; the next enemy that is on deck to be entering the playfield
WaveNumEn           ds       1                            ; the # of enemies that have a definition (wave1 has 7 enemies defined. each with movement tables, sprite shapes, etc)
WaveLstEnK          ds       1                            ; the last enemy # to be killed (this is needed for replacement)
WaveType            ds       1                            ; = 0 then we're on non-squadron enemies, = 9 then we're on squadron enemies
WaveEnWait          ds       1                            ; how long for each enemy to wait before entering the screen
WaveEnWait2         ds       1
WaveEnWait3         ds       1
WaveEnWait4         ds       1
WaveEnWait5         ds       1
WaveEnWait6         ds       1
WaveEnWait7         ds       1
WaveEnWait8         ds       1
WaveEnWait9         ds       1
WaveEnWait10        ds       1
WaveEnWait11        ds       1
WaveEnWait12        ds       1
WaveEnWait13        ds       1
WaveEnWait14        ds       1
;WaveEnWait15        ds       1
WaveMultiSptr       ds       28
;WaveMultiSptr2      ds       26       
                           ; each points to a squadron movement table (used for waves that have the WaveSubDraw populated)
WaveSubDraw         ds       1                            ; decides which draw routine to use for enemies in this wave (Draw_VLc or mov_draw_vlc_a)
; bit 1     : 0 = then use Draw_VLc, 1 = use Mov_Draw_VLc_a
; bit 2-4   : open
; bit 5     : 1 = then use the sprite table for each enemy in wave declarations
; bit 6     : 0 = regular collision detection, 1 = smaller coll det, 3 = smallest
WaveSqAnims         ds       1                            ; used for sprite animations (# of animations)
WaveSqFrames        ds       1                            ; how many frames to draw before switching animation sprites
WaveSqFraCtr        ds       1                            ; which frame we currently are on with animations
WaveSqSprites       ds       2                            ; points to a table of all animations to cycle thru
WaveAnimSpr         ds       2                            ; (for 3-sprite enemy) points to a sprite table to animate thru
WaveSqCurPtr        ds       1                            ; which sprite # we are in regarding animation of Squadron sprite
WaveSqAcSpr         ds       2                            ; this is the EXACT sprite vector list we're drawing when animating 3-sprite enemy
CurrEnY             ds       1                            ; (in the loop) the y-coordinate of the current enemy 
CurrEnX             ds       1                            ; (in the loop) the x-coordinate of the current enemy 
CurrEnInfo          ds       1                            ; (in the loop) EnemyInfo1, 2, 3, etc is copied into this
CurrMovTbl          ds       2                            ; depending on what movement table CurrEnInfo points to, this point to #eSprite1, 2, 3, etc 
CurrSprList         ds       2                            ; will contain a pointer to eSpriteLists (eSprite1List, eSprite2List, etc)
CurrShSpeed         ds       1                            ; how fast will Squadron ship shoot (0=slow, 1=medium, 2=fast)
CurrVol             ds       1
CurrTone            ds       1
BulletPlDir         ds       1                            ; each player bullet on the screen can be going left or right on PLATFORM levels
; bit 1 = 0 then player bullet #1 is going to the left, 1 = going to the right
; bit 2 = 0 then player bullet #2 is going to the left, 1 = going to the right
; bit 3 = 0 then player bullet #3 is going to the left, 1 = going to the right
; bit 4 = unused
; bit 5 = 0 then enemy bullet #1 is going to the left, 1 = going to right
; bit 6 = 0 then enemy bullet #2 is going to the left, 1 = going to right
; bit 7 = 0 then enemy bullet #3 is going to the left, 1 = going to right
; bit 8 = unused
BulletC1Y           ds       1                            ; used to temporarily store each bullet y coord during the "moving" loop
BulletC1X           ds       1                            ; used to temporarily store each bullet x coord during the "moving" loop
Bullet1YPos         ds       1
Bullet2YPos         ds       1
Bullet3YPos         ds       1
Bullet1XPos         ds       1
Bullet2XPos         ds       1
Bullet3XPos         ds       1
BitPickedUp         ds       1                           ; = 1 then we picked up a bit that triggered a rule
BitsAllPicked       ds       1                           ; a counter while the "all_bits_pickedup" sound plays
BitY1               ds       1
BitY2               ds       1
BitY3               ds       1
BitY4               ds       1
;BitY5               ds       1
BitX1               ds       1
BitX2               ds       1
BitX3               ds       1
BitX4               ds       1
BitSprite           ds       4
ShotCtr             ds       1                            ; when Tom has a gun and shoots, this counts down how long the gun shows in his hand
SpawnDec            ds       1                            ; how much to decrement SpawnCounter during spawn or despawn
SpawnEnd            ds       1                            ; when to stop the animations
SpawnType           ds       1                            ; used to check when a Spawn completes - did an enemy (0) or Tom (1) just spawn into place
SpawnHeight         ds       1
DespawnEnemy        ds       1                            ; if > 0 then this represents the enemy that is despawning (from enemyhealth)
SpawnDelay          ds       1
SpawnCounter        ds       1
SpawnLoopCtr        ds       1                            ; pointer to where we are in the spawn sounds
SpawnY              ds       1                            ; next two are the location of the spawn which is happening for Tom or enemy
SpawnX              ds       1
AttrCtr             ds       1                            ; when > 0 then we're showing on screen instructions
AttrCtr2            ds       1                            ; 16-bit counter
AttrMode            ds       1                            ; this determines what exactly we are showing re: on screen instructions
; 1 = showing shoot alients
; 2 = showing pick up fuel 
; 3 - show getting gun and shooting a machine champ
; 4 - showing picking up bits and exiting
animationCounter    ds       2
TitleSong           ds       1                            ; this will be populated when we're playing Space Oddity
TitleSongPtr        ds       2                            ; will always point to "Space_Oddity_Sng"
VolPos              ds       1
LoopCtr             ds       1                            ; generic counter used for many loops
nmi                 ds       1
nmiR                ds       1                            ; used for Recognizers
MaxRecNum           ds       1                            ; how many recs are allowed on screen at once (set in initPlatform)
maxBullH            ds       1
MusicState          ds       1                            ; used to flag pieces of music being played or not
SpacePlatLvl        ds       1                            ; first 4 bits are which level (0-15) the space level player is on and the other 4 bits are which platform level
ShipPos             ds       1                            ; this counter is used to know when to switch player space ship patterns (the phoenix animation) 
GSRScale            ds       1                            ; the Gun Shots remaining bar at lower left of Platform screens
GSRScaleSav         ds       1                            ; this is for carrover on how much Power Up (shields or missiles) the player has when leaving SS and going to platformer
GSRemaining1        ds       1                            ; these next 5 are used for the Gun Shots Remaining bar that appears when Tom has the gun
GSRemaining2        ds       1
GSRemaining3        ds       1
GSRemaining4        ds       1
GenericFlag         ds       1                            ; each of the eight bits are used s flags (see below) 
; Bit 1 = if 0, the player ship sprite is the big one, else it's the little one
; Bit 2 = if 1, then player didn't move up or down (used for the ladder climbing logic)
;       = (space shooter) if = 1 then all enemies in a squadron are on the screen
; Bit 3 = if 1, then we are in Space Shooter, if 0 then we are on platform
; Bit 4 = if 0, then the Spawn going on is for Tom, if 1, then the current spawn is for enemy (this is used to know what to do when Spawn completes)
;       = (space shooter) if = 1 then Drop Fuel is on screen
; Bit 5 = (platform) if 1, then we've done the "8" record (first entry) of the movement pattern. 0 means we have not.
;       = (space shooter) if 1, then we've used the Warp for the current ship, 0 = Warp is available
; Bit 6 = (space shooter) if 0, then we are incrementing first 8 stars
;       = (platform) if 1, then we've used the Warp for the current ship (Tom), 0 = Warp is available
; Bit 7 = (space shooter) if 0, then we are incrementing second 8 stars
; Bit 8 = (space shooter) if 0, then we are incrementing third 7 stars
; 8 4 2 1 
; 1 1 0 1

enemyDrwNmi         ds       1                            ; used to divide the drawing of enemies in half to be more efficient
Lives               ds       1
DisplayLives        ds       1
Cv1Ctr              ds       1                            ; brightness of "PAUSE" text
FrameCtrEI          ds       1
ExtraMan            ds       7                            ; this holds the score player must get for an extra man (5000, 10000, etc...it increments by 5k everytime he gets an extra man)
ExtraManSnd         ds       1
BonusPts            ds       7                            ; this is for showing the bonus points awarded at end of each platform
buffer              ds       1
Vec_Text_Width_neg  EQU      buffer                       ; variable used in own printing routines 
print_space         EQU      Vec_Text_Width_neg+2         ; buffer for draw numbers
ym_ram              ds       166                          ; to use YM PLAYER (arkos tracker) you need 166 bytes
;RamUsed             ds       1

