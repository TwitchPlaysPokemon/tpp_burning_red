RockTunnelPokecenter_Script:
	call Serial_TryEstablishingExternallyClockedConnection
	ld a, ROUTE_10
	ld [wLastBlackoutMap], a
	jp EnableAutoTextBoxDrawing

RockTunnelPokecenter_TextPointers:
	dw RockTunnelHealNurseText
	dw RockTunnelPokecenterText2
	dw RockTunnelPokecenterText3
	dw RockTunnelTradeNurseText

RockTunnelHealNurseText:
	db $ff

RockTunnelPokecenterText2:
	TX_FAR _RockTunnelPokecenterText2
	db "@"

RockTunnelPokecenterText3:
	TX_FAR _RockTunnelPokecenterText3
	db "@"

RockTunnelTradeNurseText:
	db $f6
