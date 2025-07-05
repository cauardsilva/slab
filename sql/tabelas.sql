CREATE TABLE SubscriptionTypes (
	SubscriptionTypeId uuid NOT NULL,
	SubscriptionTypeName varchar(128) NOT NULL,
	Price numeric NOT NULL,
	MaxWorkspaces integer,
	MaxReactions integer,
	MaxAttachmentSize integer,
	Duration interval NOT NULL,
	Valid boolean NOT NULL,
	PRIMARY KEY (SubscriptionTypeId)
);

CREATE TABLE Users (
	UserId uuid NOT NULL,
	UserName varchar(128) NOT NULL,
	DisplayName varchar(128),
	UserEmail varchar(128) NOT NULL,
	CreatedAt timestamp NOT NULL,
	SecretValue varchar(64) NOT NULL,
	SecretType varchar(64) NOT NULL, -- definir os tipos
	SecretExpirationDate timestamp,
	SubscriptionTypeId uuid NOT NULL,
	SubscriptionCreatedAt timestamp NOT NULL,
	numCreatedWorkspaces integer NOT NULL,
	numSentReactions integer NOT NULL,
	PRIMARY KEY (UserId),
	UNIQUE(UserName),
	UNIQUE(UserEmail),
	UNIQUE(UserId, SecretType),
	FOREIGN KEY (SubscriptionTypeId) REFERENCES SubscriptionTypes
);


CREATE TABLE Workspaces (
	WorkspaceId uuid NOT NULL,
	WorkspaceName varchar(128) NOT NULL,
	OwnerUserId uuid NOT NULL,
	CreatedAt timestamp NOT NULL,
	PRIMARY KEY (WorkspaceId),
	FOREIGN KEY (OwnerUserId) REFERENCES Users
);


CREATE TABLE Channels (
	ChannelId uuid NOT NULL,
	WorkspaceId uuid NOT NULL,
	ChannelType varchar(128) NOT NULL, -- definir os tipos
	ChannelName varchar(128) NOT NULL,
	CreatedAt timestamp NOT NULL,
	PRIMARY KEY (ChannelId),
	FOREIGN KEY (WorkspaceId) REFERENCES Workspaces,
	UNIQUE (WorkspaceId, ChannelName)
);


CREATE TABLE ChannelMemberships (
	ChannelId uuid NOT NULL,
	UserId uuid NOT NULL,
	PRIMARY KEY (ChannelId, UserId),
	FOREIGN KEY (ChannelId) REFERENCES Channels,
	FOREIGN KEY (UserId) REFERENCES Users
);

CREATE TABLE WorkspaceMemberships (
	WorkspaceId uuid NOT NULL,
	UserId uuid NOT NULL,
	PRIMARY KEY (WorkspaceId, UserId),
	FOREIGN KEY (WorkspaceId) REFERENCES Workspaces,
	FOREIGN KEY (UserId) REFERENCES Users
);


CREATE TABLE Messages (
	MessageId uuid NOT NULL,
	ChannelId uuid NOT NULL,
	SenderUserId uuid NOT NULL,
	Content text NOT NULL,
	CreatedAt timestamp NOT NULL,
	ParentMessageId uuid,
	PRIMARY KEY (MessageId),
	FOREIGN KEY (ChannelId) REFERENCES Channels,
	FOREIGN KEY (SenderUserId) REFERENCES Users,
	FOREIGN KEY (ParentMessageId) REFERENCES Messages
);

CREATE TABLE UserMentions (
	UserId uuid NOT NULL,
	MessageId uuid NOT NULL,
	PRIMARY KEY (UserId, MessageId),
	FOREIGN KEY (UserId) REFERENCES Users,
	FOREIGN KEY (MessageId) REFERENCES Messages
);

CREATE TABLE Attachments (
	AttachmentId uuid NOT NULL,
	MessageId uuid NOT NULL,
	Content bytea NOT NULL,
	ContentType varchar(32) NOT NULL,
	PRIMARY KEY (AttachmentId),
	FOREIGN KEY (MessageId) REFERENCES Messages
);

CREATE TABLE Reactions (
	ReactionId uuid NOT NULL,
	Emoji varchar(16) NOT NULL,
	CreatedAt timestamp NOT NULL,
	MessageId uuid NOT NULL,
	UserId uuid NOT NULL,
	PRIMARY KEY (ReactionId),
	FOREIGN KEY (MessageId) REFERENCES Messages,
	FOREIGN KEY (UserId) REFERENCES Users
);