-- Subscription Types
INSERT INTO SubscriptionTypes (SubscriptionTypeId, SubscriptionTypeName, Price, MaxWorkspaces, MaxReactions, MaxAttachmentSize, Duration, Valid) VALUES
('a1b2c3d4-e5f6-47a8-9b0c-1d2e3f4a5b6c', 'Free', 0.0, 1, 10, 50, '1 month', true),
('b2c3d4e5-f6a7-48b9-c0d1-e2f3a4b5c6d7', 'Silver', 14.99, 5, 30, 150, '1 month', true),
('c3d4e5f6-a7b8-49c0-d1e2-f3a4b5c6d7e8', 'Gold', 24.99, 20, 100, 300, '1 month', true),
('d4e5f6a7-b8c9-4a0b-1c2d-3e4f5a6b7c8d', 'Enterprise', 99.99, 100, 500, 1000, '1 month', true);


-- Users
INSERT INTO Users (UserId, UserName, DisplayName, UserEmail, CreatedAt, SecretValue, SecretType, SecretExpirationDate, SubscriptionTypeId, SubscriptionCreatedAt, NumCreatedWorkspaces, NumSentReactions) VALUES
('01020304-0506-4708-890a-0b0c0d0e0f10', 'Luquinhas', '_Luquinhas1337!', 'luquinhas@example.com', NOW(), 'pass123', 'UnhashedPassword', NOW() + INTERVAL '1 year', 'b2c3d4e5-f6a7-48b9-c0d1-e2f3a4b5c6d7', NOW(), 1, 5),
('11121314-1516-4718-891a-1b1c1d1e1f20', 'Caua', NULL, 'caua@example.com', NOW(), 'pass456', 'UnhashedPassword', NOW() + INTERVAL '1 year', 'a1b2c3d4-e5f6-47a8-9b0c-1d2e3f4a5b6c', NOW(), 0, 2),
('21222324-2526-4728-892a-2b2c2d2e2f30', 'AnaSilva', 'Ana S.', 'ana@example.com', NOW(), 'anasecret', 'HashedPassword', NOW() + INTERVAL '2 years', 'a1b2c3d4-e5f6-47a8-9b0c-1d2e3f4a5b6c', NOW(), 0, 0),
('31323334-3536-4738-893a-3b3c3d3e3f40', 'PedroXP', 'Pedro Gaming', 'pedro@example.com', NOW(), 'gamepass', 'UnhashedPassword', NOW() + INTERVAL '6 months', 'c3d4e5f6-a7b8-49c0-d1e2-f3a4b5c6d7e8', NOW(), 0, 15),
('41424344-4546-4748-894a-4b4c4d4e4f50', 'MarianaC', NULL, 'mariana@example.com', NOW(), 'maripass', 'HashedPassword', NOW() + INTERVAL '1 year', 'b2c3d4e5-f6a7-48b9-c0d1-e2f3a4b5c6d7', NOW(), 0, 8),
('55424344-4546-4748-894a-4b4c4d4e4f50', 'ademarc', 'ADEMAR COSTA ;)', 'itsademarcostica@example.com', NOW(), 'maripass', 'HashedPassword', NOW() + INTERVAL '1 year', 'b2c3d4e5-f6a7-48b9-c0d1-e2f3a4b5c6d7', NOW(), 0, 8),
('99020304-0506-4708-890a-0b0c0d0e0f10', 'LuquinhasEnterpriseEdition', 'LuquinhasTM', 'luquinhas@enterprise.com', NOW(), '123', 'UnhashedPassword', NOW() + INTERVAL '1 year', 'd4e5f6a7-b8c9-4a0b-1c2d-3e4f5a6b7c8d', NOW(), 1, 5);

-- Workspaces
INSERT INTO Workspaces (WorkspaceId, WorkspaceName, OwnerUserId, CreatedAt) VALUES
('0a1a2a3a-4b5b-4c6c-8d7d-e8f9a0b1c2d3', 'Grupo do Luquinhas', '01020304-0506-4708-890a-0b0c0d0e0f10', NOW()),
('1a2a3a4a-5b6b-4c7c-8d8d-e9f0a1b2c3d4', 'Projeto Alpha', '01020304-0506-4708-890a-0b0c0d0e0f10', NOW()),
('2a3a4a5a-6b7b-4c8c-8d9d-eaf1a2b3c4d5', 'Dev Team', '31323334-3536-4738-893a-3b3c3d3e3f40', NOW()),
('3a4a5a6a-7b8b-4c9c-8dad-ebf2a3b4c5d6', 'Marketing Dept', '41424344-4546-4748-894a-4b4c4d4e4f50', NOW()),
('999a5a6a-7b8b-4c9c-8dad-ebf2a3b4c5d6', 'luquinhas company 2025', '99020304-0506-4708-890a-0b0c0d0e0f10', NOW());

-- Channels
INSERT INTO Channels (ChannelId, WorkspaceId, ChannelType, ChannelName, CreatedAt) VALUES
('0e1e2e3e-4f5f-4a6a-8b7b-c9d0e1f2a3b4', '0a1a2a3a-4b5b-4c6c-8d7d-e8f9a0b1c2d3', 'public', 'conversas-gerais', NOW()),
('1e2e3e4e-5f6f-4b7b-8c8c-d0e1f2a3b4c5', '0a1a2a3a-4b5b-4c6c-8d7d-e8f9a0b1c2d3', 'private', 'canal-privado-luquinhas', NOW()),
('2e3e4e5e-6f7f-4c8c-8d9d-eaf1a2b3c4d6', '1a2a3a4a-5b6b-4c7c-8d8d-e9f0a1b2c3d4', 'public', 'geral-alpha', NOW()),
('3e4e5e6e-7f8f-4d9d-8dae-ebf2a3b4c5d7', '2a3a4a5a-6b7b-4c8c-8d9d-eaf1a2b3c4d5', 'public', 'dev-discussions', NOW()),
('4e5e6e7e-8f9f-4e0e-8daf-ecf3a4b5c6d8', '2a3a4a5a-6b7b-4c8c-8d9d-eaf1a2b3c4d5', 'private', 'code-review', NOW());


-- ChannelMemberships
INSERT INTO ChannelMemberships (ChannelId, UserId) VALUES
('0e1e2e3e-4f5f-4a6a-8b7b-c9d0e1f2a3b4', '01020304-0506-4708-890a-0b0c0d0e0f10'),
('0e1e2e3e-4f5f-4a6a-8b7b-c9d0e1f2a3b4', '11121314-1516-4718-891a-1b1c1d1e1f20'),
('0e1e2e3e-4f5f-4a6a-8b7b-c9d0e1f2a3b4', '21222324-2526-4728-892a-2b2c2d2e2f30'),
('1e2e3e4e-5f6f-4b7b-8c8c-d0e1f2a3b4c5', '01020304-0506-4708-890a-0b0c0d0e0f10'),
('2e3e4e5e-6f7f-4c8c-8d9d-eaf1a2b3c4d6', '01020304-0506-4708-890a-0b0c0d0e0f10'),
('2e3e4e5e-6f7f-4c8c-8d9d-eaf1a2b3c4d6', '31323334-3536-4738-893a-3b3c3d3e3f40'),
('3e4e5e6e-7f8f-4d9d-8dae-ebf2a3b4c5d7', '31323334-3536-4738-893a-3b3c3d3e3f40'),
('3e4e5e6e-7f8f-4d9d-8dae-ebf2a3b4c5d7', '41424344-4546-4748-894a-4b4c4d4e4f50'),
('4e5e6e7e-8f9f-4e0e-8daf-ecf3a4b5c6d8', '31323334-3536-4738-893a-3b3c3d3e3f40');


-- WorkspaceMemberships
INSERT INTO WorkspaceMemberships (WorkspaceId, UserId) VALUES
('0a1a2a3a-4b5b-4c6c-8d7d-e8f9a0b1c2d3', '01020304-0506-4708-890a-0b0c0d0e0f10'),
('0a1a2a3a-4b5b-4c6c-8d7d-e8f9a0b1c2d3', '11121314-1516-4718-891a-1b1c1d1e1f20'),
('0a1a2a3a-4b5b-4c6c-8d7d-e8f9a0b1c2d3', '21222324-2526-4728-892a-2b2c2d2e2f30'),
('1a2a3a4a-5b6b-4c7c-8d8d-e9f0a1b2c3d4', '01020304-0506-4708-890a-0b0c0d0e0f10'),
('1a2a3a4a-5b6b-4c7c-8d8d-e9f0a1b2c3d4', '31323334-3536-4738-893a-3b3c3d3e3f40'),
('2a3a4a5a-6b7b-4c8c-8d9d-eaf1a2b3c4d5', '31323334-3536-4738-893a-3b3c3d3e3f40'),
('2a3a4a5a-6b7b-4c8c-8d9d-eaf1a2b3c4d5', '41424344-4546-4748-894a-4b4c4d4e4f50'),
('2a3a4a5a-6b7b-4c8c-8d9d-eaf1a2b3c4d5', '11121314-1516-4718-891a-1b1c1d1e1f20'),
('2a3a4a5a-6b7b-4c8c-8d9d-eaf1a2b3c4d5', '01020304-0506-4708-890a-0b0c0d0e0f10');



-- Messages
INSERT INTO Messages (MessageId, ChannelId, SenderUserId, Content, CreatedAt, ParentMessageId) VALUES
('0f1f2f3f-4a5a-4b6b-8c7c-d8e9f0a1b2c3', '0e1e2e3e-4f5f-4a6a-8b7b-c9d0e1f2a3b4', '11121314-1516-4718-891a-1b1c1d1e1f20', 'Opa e ai Luquinhas, saca só o desenho que eu fiz!', NOW() - INTERVAL '2 days', NULL),
('1f2f3f4f-5a6a-4b7b-8c8c-d9e0f1a2b3c4', '0e1e2e3e-4f5f-4a6a-8b7b-c9d0e1f2a3b4', '01020304-0506-4708-890a-0b0c0d0e0f10', 'Caramba Caua ficou muito irado!', NOW() - INTERVAL '1 day', '0f1f2f3f-4a5a-4b6b-8c7c-d8e9f0a1b2c3'),
('2f3f4f5f-6a7a-4c8c-8d9d-eaf1a2b3c4d5', '1e2e3e4e-5f6f-4b7b-8c8c-d0e1f2a3b4c5', '01020304-0506-4708-890a-0b0c0d0e0f10', 'Lembrar de comprar maçã e banana no seu Pedro.', NOW(), NULL),
('3f4f5f6f-7a8a-4d9d-8dae-ebf2a3b4c5d6', '2e3e4e5e-6f7f-4c8c-8d9d-eaf1a2b3c4d6', '31323334-3536-4738-893a-3b3c3d3e3f40', 'Olá a todos! Alguma atualização no projeto?', NOW() - INTERVAL '1 hour', NULL),
('4f5f6f7f-8a9a-4e0e-8daf-ecf3a4b5c6d7', '2e3e4e5e-6f7f-4c8c-8d9d-eaf1a2b3c4d6', '01020304-0506-4708-890a-0b0c0d0e0f10', 'Sim, estamos progredindo bem.', NOW() - INTERVAL '30 minutes', '3f4f5f6f-7a8a-4d9d-8dae-ebf2a3b4c5d6'),
('5f6f7f8f-9a0a-4f1f-8db0-edf4a5b6c7d8', '3e4e5e6e-7f8f-4d9d-8dae-ebf2a3b4c5d7', '41424344-4546-4748-894a-4b4c4d4e4f50', 'Bom dia equipe!', NOW() - INTERVAL '5 hours', NULL);


-- UserMentions
INSERT INTO UserMentions (UserId, MessageId) VALUES
('01020304-0506-4708-890a-0b0c0d0e0f10', '0f1f2f3f-4a5a-4b6b-8c7c-d8e9f0a1b2c3'),
('31323334-3536-4738-893a-3b3c3d3e3f40', '4f5f6f7f-8a9a-4e0e-8daf-ecf3a4b5c6d7'),
('11121314-1516-4718-891a-1b1c1d1e1f20', '2f3f4f5f-6a7a-4c8c-8d9d-eaf1a2b3c4d5'),
('41424344-4546-4748-894a-4b4c4d4e4f50', '3f4f5f6f-7a8a-4d9d-8dae-ebf2a3b4c5d6');


-- Attachments
INSERT INTO Attachments (AttachmentId, MessageId, Content, ContentType) VALUES
('00a1b2c3-d4e5-4f6f-8a7a-b8c9d0e1f2a3', '0f1f2f3f-4a5a-4b6b-8c7c-d8e9f0a1b2c3', decode('89504e470d0a1a0a0000000d49484452000000100000001008060000001f15c489000000017352474200aece1ce90000000467414d410000b18f0bfc6105000000097048597300000e1600000e16019574c8830000000774494d4507e101010101014e7a83680000000c4944415478daedc10101000000c2a0f74f670000000049454e44ae426082', 'hex'), 'image/png'),
('10b2c3d4-e5f6-4a7a-8b8b-c9d0e1f2a3b4', '0f1f2f3f-4a5a-4b6b-8c7c-d8e9f0a1b2c3', decode('4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c69717561206164206d696e696d2076656e69616d', 'hex'), 'text/plain'),
('20c3d4e5-f6a7-4b8b-8c9c-d0e1f2a3b4c5', '3f4f5f6f-7a8a-4d9d-8dae-ebf2a3b4c5d6', decode('0000001a6674797069736f6d0000020069736f6d69736f3261766331', 'hex'), 'video/mp4'),
('30d4e5f6-a7b8-4c9c-8dad-e0f1a2b3c4d6', '5f6f7f8f-9a0a-4f1f-8db0-edf4a5b6c7d8', decode('48656c6c6f2066726f6d204d617269616e612773206174746163686d656e74', 'hex'), 'text/plain');


-- Reactions
INSERT INTO Reactions (ReactionId, Emoji, CreatedAt, MessageId, UserId) VALUES
('000a1a2a-3b4b-4c5c-8d6d-e7f8a9b0c1d2', '👍', NOW() - INTERVAL '1 day', '0f1f2f3f-4a5a-4b6b-8c7c-d8e9f0a1b2c3', '01020304-0506-4708-890a-0b0c0d0e0f10'),
('101a2a3a-4b5b-4c6c-8d7d-e8f9a0b1c2d3', '❤️', NOW() - INTERVAL '1 hour', '1f2f3f4f-5a6a-4b7b-8c8c-d9e0f1a2b3c4', '11121314-1516-4718-891a-1b1c1d1e1f20'),
('202a3a4a-5b6b-4c7c-8d8d-e9f0a1b2c3d4', '🎉', NOW() - INTERVAL '30 minutes', '3f4f5f6f-7a8a-4d9d-8dae-ebf2a3b4c5d6', '41424344-4546-4748-894a-4b4c4d4e4f50'),
('303a4a5a-6b7b-4c8c-8d9d-eaf1a2b3c4d5', '😂', NOW() - INTERVAL '10 minutes', '3f4f5f6f-7a8a-4d9d-8dae-ebf2a3b4c5d6', '01020304-0506-4708-890a-0b0c0d0e0f10');