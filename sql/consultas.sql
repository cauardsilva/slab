-- Seleciona rendimento total de cada tipo de assinatura válida em cada workspace, em relação a suas respectivas durações, para fins de estatística. c
SELECT 
	st.SubscriptionTypeId,
	wkm.WorkspaceId,
	SUM(st.Price) as Subscription_Type_Revenue,
	st.Duration 
FROM Users u
	JOIN WorkspaceMemberships wkm ON u.UserId = wkm.UserId
	JOIN SubscriptionTypes st ON u.SubscriptionTypeId = st.SubscriptionTypeId
	WHERE st.Valid = true
	GROUP BY st.SubscriptionTypeId, wkm.WorkspaceId
	ORDER BY Subscription_Type_Revenue DESC, st.Duration ASC;


-- Seleciona tamanho total de anexos enviados em cada canal para fins de limpeza de armazenamento. c
SELECT
	c.ChannelName,
	SUM(OCTET_LENGTH(att.Content)) AS TotalAttachmentSize
FROM Attachments att
	JOIN Messages m ON att.MessageId = m.MessageId
	JOIN Channels c ON m.ChannelId = c.ChannelId
	GROUP BY c.ChannelId
	ORDER BY TotalAttachmentSize;

-- Seleciona os nomes de usuários que tiveram pelo menos uma mensagem no canal de id @channel_id para fins de exibição de usuários ativos na interface da plataforma. c
SELECT c.ChannelName, u.UserName FROM Users u
	JOIN Messages m on m.SenderUserId = u.UserId
	JOIN Channels c on c.ChannelId = m.ChannelId
	WHERE m.ChannelId = '0e1e2e3e-4f5f-4a6a-8b7b-c9d0e1f2a3b4'
	GROUP BY u.UserId, c.ChannelId HAVING COUNT(distinct m.MessageId) > 0;

		
-- Seleciona os usuários da plataforma que possuem assinatura paga mas ainda não fazem parte de nenhum workspace. d
SELECT u.UserId, u.UserName, st.SubscriptionTypeName
FROM Users u
	JOIN SubscriptionTypes st ON u.SubscriptionTypeId = st.SubscriptionTypeId
	WHERE st.Price > 0
		AND NOT EXISTS (
			SELECT 1
			FROM WorkspaceMemberships wkm
				WHERE wkm.UserId = u.UserId
		)
	ORDER BY u.UserName;

-- Seleciona todos os usuários que estão em todos canais públicos que o usuário ‘Caua’ e
SELECT u1.UserName FROM Users u1
	WHERE NOT EXISTS (
		SELECT 1 FROM Channels c
			JOIN ChannelMemberships cm ON c.ChannelId = cm.ChannelId
			JOIN Users u2 ON u2.UserId = cm.UserId
			WHERE c.ChannelType = 'public'
				AND u2.UserName = 'Caua'
				AND NOT EXISTS (
					SELECT 1 FROM ChannelMemberships cm1
						where cm.ChannelId = cm1.ChannelId
							AND cm1.UserId = u1.UserId
				)
	);


-- Visão ChannelUserMessages, que agrega todas as informações sobre o contexto das mensagens enviadas. f
CREATE VIEW ChannelUserMessages AS
SELECT
	u.UserId AS SenderUserId,
	u.UserName AS SenderUserName,
	u.DisplayName AS SenderDisplayName,
	c.ChannelId,
	c.ChannelName,
	c.ChannelType,
	m.MessageId,
	m.Content AS MessageContent,
	m.CreatedAt AS MessageSentAt
FROM Channels c
JOIN Messages m ON c.ChannelId = m.ChannelId
JOIN Users u ON m.SenderUserId = u.UserId;

-- Utiliza a view ChannelUserMessages para buscar o número de mensagens por canais de um dado workspace f
SELECT cum.ChannelName, COUNT(cum.MessageId) AS TotalMessages
FROM ChannelUserMessages cum
	JOIN Channels c on c.ChannelId = cum.ChannelId
	JOIN Workspaces w ON w.WorkspaceId = c.WorkspaceId
	WHERE w.WorkspaceId = '0a1a2a3a-4b5b-4c6c-8d7d-e8f9a0b1c2d3'
	GROUP BY cum.ChannelName
	ORDER BY TotalMessages DESC
	LIMIT 5;

-- Utiliza a view ChannelUserMessages para buscar a última mensagem de cada canal e seu nome. f d
SELECT ChannelName, MessageContent FROM
(
	SELECT *, ROW_NUMBER() OVER (PARTITION BY ChannelName ORDER BY MessageSentAt DESC) as RowNumber FROM (
		SELECT ChannelName, MessageContent, MessageSentAt
		FROM ChannelUserMessages

		UNION
	
		SELECT c.ChannelName, r.Emoji AS MessageContent, r.CreatedAt AS MessageSentAt
		FROM Reactions r
		JOIN Messages m ON r.MessageId = m.MessageId
		JOIN Channels c ON m.ChannelId = c.ChannelId
	)
) WHERE RowNumber = 1;

-- Busca, para cada usuário, quantas interações de outros usuários suas mensagens tiveram (sendo consideradas interações as respostas e reações as mensagens)
SELECT u.UserName, COUNT(*) as interaction_count FROM Users u
	JOIN Messages m ON m.SenderUserId = u.UserId
	JOIN Messages tm on tm.ParentMessageId = m.MessageId
	JOIN Reactions r on r.MessageId = m.MessageId
	GROUP BY u.UserId;

-- Seleciona usuários com tipo de assinatura enterprise e que são donos de workspaces, mas que têm senhas inseguras (curtas e armazenadas sem serem hasheadas no banco).
SELECT u.UserId, u.UserName
FROM Users u
	JOIN SubscriptionTypes st ON u.SubscriptionTypeId = st.SubscriptionTypeId
	WHERE u.SecretType = 'UnhashedPassword' 
		AND	LENGTH(u.SecretValue) <= 10 
		AND st.SubscriptionTypeName LIKE '%Enterprise%' 
		AND EXISTS (
    		SELECT 1
			FROM Workspaces w 
				WHERE u.UserId = w.OwnerUserId
		);

-- Seleciona o top 10 de usuarios mais engajados na plataforma no ultimo dia.
SELECT
    u.UserName,
    COUNT(DISTINCT m.MessageId) AS MessagesSentToday,
    COUNT(DISTINCT r.ReactionId) AS ReactionsGivenToday
FROM Users u
LEFT JOIN Messages m ON u.UserId = m.SenderUserId
LEFT JOIN Reactions r ON u.UserId = r.UserId
WHERE
    m.CreatedAt::date = CURRENT_DATE OR r.CreatedAt::date = CURRENT_DATE
GROUP BY
    u.UserName
ORDER BY
    (COUNT(DISTINCT m.MessageId) + COUNT(DISTINCT r.ReactionId)) DESC
LIMIT 10;
	