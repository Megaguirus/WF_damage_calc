\[ base damage \] \*

\[ 1 + pure damage mods \] \* 

\[ 1 + base critical chance \* ( 1 + critical chance mods ) \* ( critical multiplier \* ( 1 + critical damage mods ) - 1 ) \] \*

\[ 1 + faction mods \] \*

\[ 1 + elemental mods + impact mods \* impact proprtion + slash mods \* slash proportion + puncture mods \* puncture proportion \] \*

\[ base multishot \* ( 1 + multishot mods ) \]   

total crit chance = base crit chance \* ( 1 + relative bonuses ) + flat bonuses 

* * *

quantization

scale = total damage / 16

quantized damage type value = round ( total damage type / scale ) \* scale

total damage = 100 (30 impact, 30 puncture, 40 slash)

scale = 100 / 16 = 6.25

quantized impact value = round ( 30 / 6.25 ) \* 6.25 = 31.25

quantized puncture value = round ( 30 / 6.25 ) \* 6.25 = 31.25

quantized slash value = round ( 40 / 6.25 ) \* 6.25 = 37.5

total quantized damage = 100 

damage against an infested = round ( 31.25 + 31.25 + 37.5 \* 1.5) = 119

total damage = 75 (18.8 puncture 33.8 slash 22.5 impact)

scale = 75 / 16 = 4.6875

quantized puncture value = round ( 18.8 / 4.6875 ) x 4.6875 = 18.75

quantized slash value = round ( 33.8 / 4.6875 ) x 4.6875 = 32.8125

quantized impact value = round ( 22.5 / 4.6875 ) 4.6875 =  23.4375

total quantized damage = 75

shuffling fragment = 75 

infested charger = 91

corpus tech = 84

total damage = 781 (429.6 puncture 351.4 impact)

scale = 781 / 16 = 48.8125

quantized puncture value = round ( 429.6 / 48.8125 ) x 48.8125 = 439.3125 

quantized impact value = round ( 351.4 / 48.8125 ) x 48.8125 = 341.6875

total quantized damage = 781

shuffling fragment = 781

infested charger = 781

corpus tech = 1001

damage multipliers, like additive (serration) and multiplicative (faction) only multiply to the final quantized value and do not effect the scale 

 the example above with a maxed serration

31.25 \* ( 1 + 1.65 ) = 83

31.25 \* ( 1 + 1.65 ) = 83

37.5 x ( 1+ 1.65 ) x 1.5 = 125

total damage = 291

*   scale:

base damage / 16

*   base quantization:

for each IPS: round ( base portion / scale ) x scale 

*   elemental quantization 

for each modded element: round \[ ( sum of percentages x total base ) / scale \] x scale 

*   IPS quantization

for each modded IPS: round \[ ( sum of percentages x portion base ) / scale \] x scale

*   total damage:

base quantization + elemental quantization + IPS quantization

*   pure damage multipliers

total damage x ( sum of pure multipliers like serration)

*   crits

quantized crit multiplier = round ( crit multiplier x 4095  /  32 ) x 32 / 4095  

crit tier multiplier = 1 + crit tier x ( quantized crit multiplier - 1 )

*   DOTs

modded damage = base damage x pure damage x critical damage x faction damage 

slash = 0.35 x modded damage x ( 1 + faction damage ) x proc damage bonuses

toxin = 0.5 x modded damage x ( 1 + faction damage ) x ( 1 + toxin damage ) x proc damage bonuses

heat = 0.5 x modded damage x ( 1 + faction damage ) x ( 1 + heat damage ) x proc damage bonuses

gas = 0.5 x modded damage x ( 1 + faction damage ) x ( 1 + gas damage ) \[ valence formation ? \] x proc damage bonuses 

electric = 0.5 x modded damage x ( 1 + faction damage ) x ( 1 + electric damage ) x proc damage bonuses

*   faction multipliers
*   magnetic / viral multipliers
*   enemy armor / damage reduction

example: the purgator1

*   scale: 

( 351.45 + 429.55 ) / 16 = 48.8125

*   base quantization:

round ( impact portion / scale ) x scale = round ( 351.45 / 48.8125 ) x 48.8125 = 341.6875 impact

round ( puncture portion / scale ) x scale = round ( 429.55 / 48.8125 ) x 48.8125 = 439.3125 puncture 

*   modded elemental quantization

round \[ ( heat percentage x base ) / scale \] x scale = round \[ ( 0.9 x 781 ) / 48.8125 \] x 48.8125 = 683.375 heat

*   modded IPS quantization

round \[ ( puncture percentage x impact portion ) / scale \] x scale = round \[ ( 0.9 x 429.55 ) / 48.8125 \] x 48.8125 = 390.5 puncture 

*   total damage

341.6875 impact + 439.3125 puncture + 683.375 heat + 390.5 puncture  \= 1854.875 damage

*   pure damage multipliers

total damage w pure damage multipliers = 1854.875 x ( 1 + 1.65 ) = 4915.41875 damage

quantized crit multiplier: round (2.3 x 4095 / 32) x 32 / 4095 = 2.297435897 crit multiplier 

crit tier multiplier = 1 + crit tier x ( quantized crit multiplier - 1 ) 

tier 1 crit multiplier = 2.297435897 crit multiplier 

tier 2 crit multiplier = 3.594871794 crit multiplier

tier 3 crit multiplier = 4.892307691 crit multiplier 

crit tier || calculation || in game registration (test passed) || in game registration (test not passed)

tier 0 crit || tier 1 crit || tier 2 crit

Unmodded: 781 (781) || 1794 (1794.297436) || 2808 (2807.594871)  
Heat +90: 1464 (1464.375) || 3364 (3364.307692) || 5264 (5264.340383)  
Puncture +90: 1172 (1171.5) || 2691 (2691.446153) || 4211 (4211.392307)  
Serration +165: 2070 (2069.65) || 4755 (4754.888204) || 7440 (7440.126408)  
Heat puncture +90 +90: 1855 (1854.875) || 4261 (4261.456409) || 6667 (6668.037819)   
Heat serration +90 +165: 3881 (3880.59375) || 8915 (8915.415383) || 13950 (13950.23702)  
Puncture serration +90 +165: 3104 (3104.475) || 7132 (7312.332306) || 11160 (11160.18961)  
Heat puncture serration +90 +90 +165: 4915 (4915.41875) || 11293 (11292.85949) || 17670 (17669.54396)

*   DoT damage:

damage values outputed by the program :

 781.0, 1794.0  
rupture 1074.0, 2467.0  
piercing\_hit 1172.0, 2691.0  
rupture piercing\_hit 1464.0, 3364.0  
sawtooth\_clip 781.0, 1794.0  
rupture sawtooth\_clip 1074.0, 2467.0  
piercing\_hit sawtooth\_clip 1172.0, 2691.0  
rupture piercing\_hit sawtooth\_clip 1464.0, 3364.0  
hellfire 1464.0, 3364.0  
rupture hellfire 1757.0, 4037.0  
piercing\_hit hellfire 1855.0, 4261.0  
rupture piercing\_hit hellfire 2148.0, 4934.0  
sawtooth\_clip hellfire 1464.0, 3364.0  
rupture sawtooth\_clip hellfire 1757.0, 4037.0  
piercing\_hit sawtooth\_clip hellfire 1855.0, 4261.0  
rupture piercing\_hit sawtooth\_clip hellfire 2148.0, 4934.0  
infected\_clip 1464.0, 3364.0  
rupture infected\_clip 1757.0, 4037.0  
piercing\_hit infected\_clip 1855.0, 4261.0  
rupture piercing\_hit infected\_clip 2148.0, 4934.0  
sawtooth\_clip infected\_clip 1464.0, 3364.0  
rupture sawtooth\_clip infected\_clip 1757.0, 4037.0  
piercing\_hit sawtooth\_clip infected\_clip 1855.0, 4261.0  
rupture piercing\_hit sawtooth\_clip infected\_clip 2148.0, 4934.0  
hellfire infected\_clip 2197.0, 5046.0  
rupture hellfire infected\_clip 2489.0, 5719.0  
piercing\_hit hellfire infected\_clip 2587.0, 5944.0  
rupture piercing\_hit hellfire infected\_clip 2880.0, 6616.0  
sawtooth\_clip hellfire infected\_clip 2197.0, 5046.0  
rupture sawtooth\_clip hellfire infected\_clip 2489.0, 5719.0  
piercing\_hit sawtooth\_clip hellfire infected\_clip 2587.0, 5944.0  
rupture piercing\_hit sawtooth\_clip hellfire infected\_clip 2880.0, 6616.0  
cryo\_rounds 1464.0, 3364.0  
rupture cryo\_rounds 1757.0, 4037.0  
piercing\_hit cryo\_rounds 1855.0, 4261.0  
rupture piercing\_hit cryo\_rounds 2148.0, 4934.0  
sawtooth\_clip cryo\_rounds 1464.0, 3364.0  
rupture sawtooth\_clip cryo\_rounds 1757.0, 4037.0  
piercing\_hit sawtooth\_clip cryo\_rounds 1855.0, 4261.0  
rupture piercing\_hit sawtooth\_clip cryo\_rounds 2148.0, 4934.0  
hellfire cryo\_rounds 2197.0, 5046.0  
rupture hellfire cryo\_rounds 2489.0, 5719.0  
piercing\_hit hellfire cryo\_rounds 2587.0, 5944.0  
rupture piercing\_hit hellfire cryo\_rounds 2880.0, 6616.0  
sawtooth\_clip hellfire cryo\_rounds 2197.0, 5046.0  
rupture sawtooth\_clip hellfire cryo\_rounds 2489.0, 5719.0  
piercing\_hit sawtooth\_clip hellfire cryo\_rounds 2587.0, 5944.0  
rupture piercing\_hit sawtooth\_clip hellfire cryo\_rounds 2880.0, 6616.0  
infected\_clip cryo\_rounds 2197.0, 5046.0  
rupture infected\_clip cryo\_rounds 2489.0, 5719.0  
piercing\_hit infected\_clip cryo\_rounds 2587.0, 5944.0  
rupture piercing\_hit infected\_clip cryo\_rounds 2880.0, 6616.0  
sawtooth\_clip infected\_clip cryo\_rounds 2197.0, 5046.0  
rupture sawtooth\_clip infected\_clip cryo\_rounds 2489.0, 5719.0  
piercing\_hit sawtooth\_clip infected\_clip cryo\_rounds 2587.0, 5944.0  
rupture piercing\_hit sawtooth\_clip infected\_clip cryo\_rounds 2880.0, 6616.0  
hellfire infected\_clip cryo\_rounds 2880.0, 6616.0  
rupture hellfire infected\_clip cryo\_rounds 3173.0, 7289.0  
piercing\_hit hellfire infected\_clip cryo\_rounds 3270.0, 7514.0  
rupture piercing\_hit hellfire infected\_clip cryo\_rounds 3563.0, 8186.0  
sawtooth\_clip hellfire infected\_clip cryo\_rounds 2880.0, 6616.0  
rupture sawtooth\_clip hellfire infected\_clip cryo\_rounds 3173.0, 7289.0  
piercing\_hit sawtooth\_clip hellfire infected\_clip cryo\_rounds 3270.0, 7514.0  
rupture piercing\_hit sawtooth\_clip hellfire infected\_clip cryo\_rounds 3563.0, 8186.0  
serration 2070.0, 4755.0  
rupture serration 2846.0, 6538.0  
piercing\_hit serration 3104.0, 7132.0  
rupture piercing\_hit serration 3881.0, 8915.0  
sawtooth\_clip serration 2070.0, 4755.0  
rupture sawtooth\_clip serration 2846.0, 6538.0  
piercing\_hit sawtooth\_clip serration 3104.0, 7132.0  
rupture piercing\_hit sawtooth\_clip serration 3881.0, 8915.0  
hellfire serration 3881.0, 8915.0  
rupture hellfire serration 4657.0, 10698.0  
piercing\_hit hellfire serration 4915.0, 11293.0    
rupture piercing\_hit hellfire serration : 5692.0, 13076.0  
sawtooth\_clip hellfire serration : 3881.0, 8915.0  
rupture sawtooth\_clip hellfire serration : 4657.0, 10698.0  
piercing\_hit sawtooth\_clip hellfire serration : 4915.0, 11293.0  
rupture piercing\_hit sawtooth\_clip hellfire serration : 5692.0, 13076.0  
infected\_clip serration : 3881.0, 8915.0  
rupture infected\_clip serration : 4657.0, 10698.0  
piercing\_hit infected\_clip serration : 4915.0, 11293.0  
rupture piercing\_hit infected\_clip serration : 5692.0, 13076.0  
sawtooth\_clip infected\_clip serration : 3881.0, 8915.0  
rupture sawtooth\_clip infected\_clip serration : 4657.0, 10698.0  
piercing\_hit sawtooth\_clip infected\_clip serration : 4915.0, 11293.0  
rupture piercing\_hit sawtooth\_clip infected\_clip serration : 5692.0, 13076.0  
hellfire infected\_clip serration : 5821.0, 13373.0  
rupture hellfire infected\_clip serration : 6597.0, 15156.0   
piercing\_hit hellfire infected\_clip serration : 6856.0, 15751.0  
rupture piercing\_hit hellfire infected\_clip serration : 7632.0, 17534.0  
sawtooth\_clip hellfire infected\_clip serration : 5821.0, 13373.0   
rupture sawtooth\_clip hellfire infected\_clip serration : 6597.0, 15156.0  
piercing\_hit sawtooth\_clip hellfire infected\_clip serration : 6856.0, 15751.0  
rupture piercing\_hit sawtooth\_clip hellfire infected\_clip serration : 7632.0, 17534.0   
cryo\_rounds serration : 3881.0, 8915.0  
rupture cryo\_rounds serration : 4657.0, 10698.0   
piercing\_hit cryo\_rounds serration : 4915.0, 11293.0  
rupture piercing\_hit cryo\_rounds serration : 5692.0, 13076.0  
sawtooth\_clip cryo\_rounds serration : 3881.0, 8915.0  
rupture sawtooth\_clip cryo\_rounds serration : 4657.0, 10698.0  
piercing\_hit sawtooth\_clip cryo\_rounds serration : 4915.0, 11293.0  
rupture piercing\_hit sawtooth\_clip cryo\_rounds serration : 5692.0, 13076.0  
hellfire cryo\_rounds serration : 5821.0, 13373.0  
rupture hellfire cryo\_rounds serration : 6597.0, 15156.0  
piercing\_hit hellfire cryo\_rounds serration : 6856.0, 15751.0  
rupture piercing\_hit hellfire cryo\_rounds serration : 7632.0, 17534.0  
sawtooth\_clip hellfire cryo\_rounds serration : 5821.0, 13373.0  
rupture sawtooth\_clip hellfire cryo\_rounds serration : 6597.0, 15156.0  
piercing\_hit sawtooth\_clip hellfire cryo\_rounds serration : 6856.0, 15751.0  
rupture piercing\_hit sawtooth\_clip hellfire cryo\_rounds serration : 7632.0, 17534.0  
infected\_clip cryo\_rounds serration : 5821.0, 13373.0  
rupture infected\_clip cryo\_rounds serration : 6597.0, 15156.0  
piercing\_hit infected\_clip cryo\_rounds serration : 6856.0, 15751.0  
rupture piercing\_hit infected\_clip cryo\_rounds serration : 7632.0, 17534.0  
sawtooth\_clip infected\_clip cryo\_rounds serration : 5821.0, 13373.0  
rupture sawtooth\_clip infected\_clip cryo\_rounds serration : 6597.0, 15156.0  
piercing\_hit sawtooth\_clip infected\_clip cryo\_rounds serration : 6856.0, 15751.0  
rupture piercing\_hit sawtooth\_clip infected\_clip cryo\_rounds serration : 7632.0, 17534.0  
hellfire infected\_clip cryo\_rounds serration : 7632.0, 17534.0  
rupture hellfire infected\_clip cryo\_rounds serration : 8408.0, 19317.0  
piercing\_hit hellfire infected\_clip cryo\_rounds serration : 8667.0, 19911.0  
rupture piercing\_hit hellfire infected\_clip cryo\_rounds serration : 9443.0, 21694.0  
sawtooth\_clip hellfire infected\_clip cryo\_rounds serration : 7632.0, 17534.0  
rupture sawtooth\_clip hellfire infected\_clip cryo\_rounds serration : \[8408.0, 19317.0  
piercing\_hit sawtooth\_clip hellfire infected\_clip cryo\_rounds serration : 8667.0, 19911.0  
rupture piercing\_hit sawtooth\_clip hellfire infected\_clip cryo\_rounds serration : 9443.0, 21694.0